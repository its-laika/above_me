use crate::{
    ogn::{Aircraft, AircraftId},
    time::get_current_timestamp,
};

use super::conversion::convert;
use super::status::Status;
use log::{debug, error};
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};
use tokio::io::{AsyncBufReadExt, BufReader, BufWriter};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::Sender,
};

/// Messages starting with a hashtag are comments (e.g. keep alive messages)
const IDENTIFIER_COMMENT: char = '#';
/// Messages starting with this sequence are connection details
const IDENTIFIER_TCP_PACKET: &str = "TCPIP*";
/// Approx. interval of keep alive messages to the server (in seconds)
const KEEPALIVE_INTERVAL_SECONDS: u64 = 60 * 10;
/// Keep alive message
const KEEPALIVE_MESSAGE: &[u8; 12] = b"#keep alive\n";

/// Configuration for connecting to an APRS server
#[derive(Deserialize)]
pub struct Config<A: ToSocketAddrs> {
    /// Address to connect to, e.g. "aprs.example.com"
    pub address: A,
    /// User name for authentication
    pub user_name: String,
    /// Password for authentication
    pub password: String,
    /// Name of the application
    pub client_id: String,
    /// APRS filter that will be applied
    pub filter: Option<String>,
}

/// Initiates a `TcpClient` that connects to an APRS server based on given `ClientConfig` and transmits incoming aircraft states.
/// Sends incoming APRS states via `status_tx`.
///
/// # Arguments
///
/// * `config` - Information on where to connect & login
/// * `status_tx` - A `Sender<String>` that will send incoming states from the server
/// * `aircraft` - Mapping of `AircraftId` => `Aircraft`, necessary for conversion
///
/// # Returns
///
/// Future that will either result to () or Error when an error occurs.
///
/// # Examples
///
/// ```
/// use ogn::{Aircraft, AircraftId};
/// use std::collections::HashMap;
/// use tokio::{spawn, sync::mpsc::channel};
///
/// let config = aprs::ClientConfig { ... };
/// let (status_tx, status_rx) = channel(32);
/// let aircraft: HashMap<AircraftId, Aircraft> = HashMap::new();
///
/// spawn(async move {
///     aprs::init(&config, status_tx, &aircraft)
///         .await
///         .expect("Client failed");
/// });
///
/// while let Some(status) = status_rx.recv().await {
///     println!("Got status: {}", status);
/// }
/// ```
pub async fn init<A: ToSocketAddrs>(
    config: &Config<A>,
    status_tx: Sender<Status>,
    aircraft: &HashMap<AircraftId, Aircraft>,
) -> Result<(), Error> {
    let mut tcp_stream = TcpStream::connect(&config.address).await?;
    let (mut read_half, mut write_half) = tcp_stream.split();

    let mut tcp_stream_reader = BufReader::new(&mut read_half);
    let mut tcp_stream_writer = BufWriter::new(&mut write_half);

    /* Login to server */
    let login_message = if let Some(filter) = &config.filter {
        format!(
            "user {} pass {} vers {} filter {}\n",
            config.user_name, config.password, config.client_id, filter
        )
    } else {
        format!(
            "user {} pass {} vers {}\n",
            config.user_name, config.password, config.client_id
        )
    };

    let mut last_keep_alive_timestamp = get_current_timestamp();

    tcp_stream_writer
        .write_all(login_message.as_bytes())
        .await?;
    tcp_stream_writer.flush().await?;

    loop {
        let mut line = String::new();

        match tcp_stream_reader.read_line(&mut line).await {
            Ok(0) => {
                debug!("Connection closed");
                return Ok(());
            }
            Ok(_) => (),
            Err(e) => {
                /* This may happen */
                error!("Error while reading line: {e}");
                continue;
            }
        };

        debug!("Got line: '{line}'");

        /* APRS server sends a keep alive ever 20 - 30 seconds. As we don't want to worry about
         * *another* async interval shit, we just check if the last keep alive was 10 - 11 mins
         * ago and, if so, send a new one. We won't run into a timeout if we're 30 seconds late,
         * so KISS FTW. */
        let current_timestamp = get_current_timestamp();
        if current_timestamp - last_keep_alive_timestamp >= KEEPALIVE_INTERVAL_SECONDS {
            last_keep_alive_timestamp = current_timestamp;

            tcp_stream_writer.write_all(KEEPALIVE_MESSAGE).await?;
            tcp_stream_writer.flush().await?;

            debug!("Sent keep alive");
        }

        if line.starts_with(IDENTIFIER_COMMENT) || line.contains(IDENTIFIER_TCP_PACKET) {
            continue;
        }

        if let Some(status) = convert(&line, aircraft) {
            if !status.aircraft.visible {
                debug!("Got message for non-visible aircraft. Discard.");
                continue;
            }

            debug!("Passing message for aircraft '{}'", status.aircraft.id);

            status_tx
                .send(status)
                .await
                .or(Err(Error::new(ErrorKind::Other, "Could not send status")))?;
        }
    }
}
