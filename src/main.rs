use api::init_api_server;
use aprs::{init_aprs_client, ClientConfig};
use ddb::fetch_aircrafts;
use tokio::sync::{mpsc, oneshot};

mod api;
mod aprs;
mod ddb;
mod mutex;
mod time;

#[tokio::main]
async fn main() {
    let aircrafts = match fetch_aircrafts().await {
        Ok(a) => a,
        Err(e) => {
            println!("Could not fetch aircraft data: {}", e);
            return;
        }
    };

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (status_tx, status_rx) = mpsc::channel(32);

    let mut server_handle = match init_api_server(&"127.0.0.1:8080", status_rx, shutdown_rx).await {
        Ok(s) => s,
        Err(e) => {
            println!("Could not start API server: {}", e);
            return;
        }
    };

    let config = ClientConfig {
        address: "...",
        user_name: "...",
        password: "...",
        filter: "...",
    };

    if let Err(e) = init_aprs_client(&config, status_tx, &aircrafts).await {
        println!("Client stopped with error: {}", e);
    }

    shutdown_tx.send(()).unwrap();

    while server_handle.join_next().await.is_some() {}
}
