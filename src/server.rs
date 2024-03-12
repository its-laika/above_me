use std::io::Error;
use std::time::Duration;

use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, ToSocketAddrs},
    spawn,
    sync::watch,
    time::interval,
};

/// Starts a dummy server that sends given message every given duration
///
/// # Arguments
///
/// * `address` - Address to bind to
/// * `message` - The message that should be sent every `repeat_interval` to every connected client
/// * `repeat_interval` - The interval in which the message should be sent
///
/// # Examples
///
/// ```
/// use std::time::Duration;
///
/// let address = "127.0.0.1:9000";
/// let message = "Message".as_bytes().to_vec();
/// let duration = Duration::from_secs(2);
///
/// start_server(address, message, duration).await;
/// ```
pub async fn start_dummy_server<A: ToSocketAddrs>(
    address: A,
    message: Vec<u8>,
    repeat_interval: Duration,
) -> Result<(), Error> {
    let listener = TcpListener::bind(address).await?;
    let (tx, rx) = watch::channel::<()>(());

    spawn(async move {
        let mut interval = interval(repeat_interval);

        loop {
            interval.tick().await;
            tx.send(()).unwrap();
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut stream_receiver = rx.clone();
        let stream_message = message.clone();

        spawn(async move {
            while let Ok(_message) = stream_receiver.changed().await {
                if socket.write_all(&stream_message).await.is_err() {
                    return;
                }
            }
        });
    }
}
