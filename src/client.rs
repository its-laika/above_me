use std::io::{Error, ErrorKind};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::{mpsc::Sender, watch::Receiver},
};

const MAX_MESSAGE_SIZE: usize = 256;
const KEEP_ALIVE_MESSAGE: &[u8; 12] = b"# keep alive";

/// Initiates a `TcpClient` that connects to given address and transmits incoming messages.
/// Will send a keep alive message based on `message_tx` changes.
///
/// # Arguments
///
/// * `address` - Address to connect to
/// * `keep_alive_rx` - A `Receiver` that determines when to send keep alive messages to
///    the server (to not lose the connection).
/// * `message_tx` - A `Sender` that will send incoming messages from the server
///
/// # Examples
///
/// ```
/// use tokio::{
///    sync::{mpsc, watch}
/// };
///
/// let address = "127.0.0.1:9000";
/// let (_keep_alive_tx, keep_alive_rx) = watch::channel(());
/// let (message_tx, _message_rx) = mpsc::channel(32);
///
/// let _ = init_tcp_client(address, keep_alive_rx, message_tx).await;
/// ```
pub async fn init_tcp_client<A: ToSocketAddrs>(
    address: A,
    mut keep_alive_rx: Receiver<()>,
    message_tx: Sender<String>,
) -> Result<(), Error> {
    let mut tcp_stream = TcpStream::connect(address).await?;

    loop {
        let data = read_limited(&mut tcp_stream).await?;
        if data.is_empty() {
            /* Connection closed. */
            return Ok(());
        }

        let message = String::from_utf8(data).or(Err(Error::new(
            ErrorKind::InvalidData,
            "Data not valid UTF-8",
        )))?;

        message_tx
            .send(message)
            .await
            .or(Err(Error::new(ErrorKind::Other, "Could not use message")))?;

        if let Ok(true) = keep_alive_rx.has_changed() {
            /* Not perfect to only send between messages but hopefully will do just fine. */
            keep_alive_rx.mark_unchanged();
            tcp_stream.write_all(KEEP_ALIVE_MESSAGE).await?;
        }
    }
}

/// Reads incoming stream data while making sure that `MAX_MESSAGE_SIZE` is not exceeded.  
/// (Note that internally `MAX_MESSAGE_SIZE` *may* be exceeded by some bytes but the range check
/// will result in an `Err`.)
///
/// # Arguments
///
/// * `tcp_stream` - The `TcpStream` to read data from
///
/// # Examples
/// ```
/// use tokio::net::TcpStream;
///
/// let mut tcp_stream = TcpStream::connect(address).await?;
///
/// let data = read_limited(&mut tcp_stream).await?;
/// ```
async fn read_limited(tcp_stream: &mut TcpStream) -> Result<Vec<u8>, Error> {
    let mut data: Vec<u8> = vec![];
    let mut buffer: [u8; 10] = [0; 10];

    'read_buffer: loop {
        let read_bytes = tcp_stream.read(&mut buffer).await?;
        data.extend_from_slice(&buffer[0..read_bytes]);

        if read_bytes < buffer.len() {
            break 'read_buffer;
        }

        if data.len() > MAX_MESSAGE_SIZE {
            return Err(Error::new(
                ErrorKind::OutOfMemory,
                format!("Exceeded max message size ({} bytes)", MAX_MESSAGE_SIZE),
            ));
        }
    }

    Ok(data)
}
