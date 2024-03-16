use crate::ddb::AircraftId;

use super::status::Status;
use super::{conversion::convert, Aircraft};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::Sender,
};

const MAX_MESSAGE_SIZE: usize = 256;
const CLIENT_ID: &str = "above_me-client 0.1";
const IDENTIFIER_COMMENT: char = '#';
const IDENTIFIER_TCP_PACKET: &str = "TCPIP*";

/// Configuration for connecting to an APRS server
///
/// # Examples
///
/// ```
/// let config = aprs::ClientConfig {
///     address: "aprs.example.com",
///     user_name: "MYC4LLS1GN",
///     password: "************",
///};
/// ```
pub struct ClientConfig<'a, A: ToSocketAddrs> {
    /// Address to connect to, e.g. "aprs.example.com"
    pub address: A,
    /// User name for authentication
    pub user_name: &'a str,
    /// Password for authentication
    pub password: &'a str,
}

/// Initiates a `TcpClient` that connects to an APRS server based on given `ClientConfig` and transmits incoming aircraft states.
/// Sends incoming APRS states via `status_tx`.
///
/// # Arguments
///
/// * `config` - Information on where to connect & login
/// * `status_tx` - A `Sender<String>` that will send incoming states from the server
/// * `aircrafts` - Mapping of `AircraftId` => `Aircraft`, necessary for conversion
///
/// # Examples
///
/// ```
/// let config = aprs::ClientConfig {
///     address: "aprs.example.com",
///     user_name: "MYC4LLS1GN",
///     password: "************",
///};
///
/// let (status_tx, _status_rx) = tokio::sync::mpsc::channel(32);
/// let aircrafts: std::collections::HashMap<ddb::AircraftId, aprs::Aircraft> = std::collections::HashMap::new();
///
/// /* Listen to `_status_rx` states here... */
///
/// let _ = aprs::init_aprs_client(&config, status_tx, &aircrafts).await?;
/// ```
pub async fn init_aprs_client<'a, A: ToSocketAddrs>(
    config: &ClientConfig<'a, A>,
    status_tx: Sender<Status>,
    aircrafts: &HashMap<AircraftId, Aircraft>,
) -> Result<(), Error> {
    let mut tcp_stream = TcpStream::connect(&config.address).await?;

    /* Login to server */
    let login_message = format!(
        "user {} pass {} vers {}",
        config.user_name, config.password, CLIENT_ID
    );

    tcp_stream.write_all(login_message.as_bytes()).await?;

    loop {
        let data = read_limited(&mut tcp_stream).await?;
        if data.is_empty() {
            /* Connection closed. */
            return Ok(());
        }

        let line = String::from_utf8(data).or(Err(Error::new(
            ErrorKind::InvalidData,
            "Data not valid UTF-8",
        )))?;

        if line.starts_with(IDENTIFIER_COMMENT) || line.starts_with(IDENTIFIER_TCP_PACKET) {
            continue;
        }

        if let Some(status) = convert(&line, aircrafts).await {
            if !status.aircraft.visible {
                continue;
            }

            status_tx
                .send(status)
                .await
                .or(Err(Error::new(ErrorKind::Other, "Could not send status")))?;
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
