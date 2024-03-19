use crate::ddb::AircraftId;

use super::status::Status;
use super::{conversion::convert, Aircraft};
use serde::Deserialize;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpStream, ToSocketAddrs},
    sync::mpsc::Sender,
};

const MAX_MESSAGE_SIZE: usize = 4096;
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

    loop {
        let data = match read_limited(&mut tcp_stream).await? {
            Some(d) => d,
            None => {
                /* Connection closed. */
                return Ok(());
            }
        };

        let lines = data.split('\n').filter(|&l| {
            !l.starts_with(IDENTIFIER_COMMENT) && !l.starts_with(IDENTIFIER_TCP_PACKET)
        });

        for line in lines {
            if let Some(status) = convert(line, aircrafts).await {
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
}

/// Reads incoming stream data as utf8 string while making sure that `MAX_MESSAGE_SIZE` is not exceeded.  
/// (Note that internally `MAX_MESSAGE_SIZE` *may* be exceeded by some bytes but the range check
/// will result in an `Err`.)
/// Returns `Ok(None)` is stream is closed
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
async fn read_limited(tcp_stream: &mut TcpStream) -> Result<Option<String>, Error> {
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

    if data.is_empty() {
        return Ok(None);
    }

    match String::from_utf8(data) {
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Data not valid UTF-8")),
        Ok(s) => Ok(Some(s)),
    }
}
