use crate::ddb::AircraftId;

use super::status::Status;
use super::{conversion::convert, Aircraft};
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{
    io::AsyncWriteExt,
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::Sender,
};

/// Messages starting with a hashtag are comments (e.g. keep alive messages)
const IDENTIFIER_COMMENT: char = '#';
/// Messages starting with this sequence are connection details
const IDENTIFIER_TCP_PACKET: &str = "TCPIP*";

/// Configuration for connecting to an APRS server
#[derive(Deserialize)]
pub struct ClientConfig<A: ToSocketAddrs> {
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
/// * `aircrafts` - Mapping of `AircraftId` => `Aircraft`, necessary for conversion
///
/// # Returns
///
/// Future that will either result to () or Error when an error occurs.
///
/// # Examples
///
/// ```
/// use aprs::Aircraft;
/// use ddb::AircraftId;
/// use std::collections::HashMap;
/// use tokio::{spawn, sync::mpsc::channel};
///
/// let config = aprs::ClientConfig { ... };
/// let (status_tx, status_rx) = channel(32);
/// let aircrafts: HashMap<AircraftId, Aircraft> = HashMap::new();
///
/// spawn(async move {
///     aprs::init_aprs_client(&config, status_tx, &aircrafts)
///         .await
///         .expect("Client failed");
/// });
///
/// while let Some(status) = status_rx.recv().await {
///     println!("Got status: {}", status);
/// }
/// ```
///
/// # Notes
///
/// Does not send keep alive messages to server as this does not seem necessary.
/// see [https://lists.tapr.org/](https://lists.tapr.org/pipermail/aprssig_lists.tapr.org/2015-April/044264.html)
pub async fn init_aprs_client<A: ToSocketAddrs>(
    config: &ClientConfig<A>,
    status_tx: Sender<Status>,
    aircrafts: &HashMap<AircraftId, Aircraft>,
) -> Result<(), Error> {
    let mut tcp_stream = TcpStream::connect(&config.address).await?;

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

    tcp_stream.write_all(login_message.as_bytes()).await?;

    let mut buf_reader = BufReader::new(&mut tcp_stream);
    loop {
        let mut line = String::new();

        if let 0 = buf_reader.read_line(&mut line).await? {
            /* Connection closed. */
            return Ok(());
        };

        if line.starts_with(IDENTIFIER_COMMENT) || line.starts_with(IDENTIFIER_TCP_PACKET) {
            continue;
        }

        if let Some(status) = convert(&line, aircrafts) {
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
