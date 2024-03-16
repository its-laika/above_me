use api::init_api_server;
use aprs::{init_aprs_client, ClientConfig};
use ddb::fetch_aircrafts;
use server::start_dummy_server;
use std::time::Duration;
use tokio::{sync::mpsc, task::JoinSet};

mod api;
mod aprs;
mod ddb;
mod mutex;
mod server;

#[tokio::main]
async fn main() {
    let aircrafts = match fetch_aircrafts().await {
        Ok(a) => a,
        Err(e) => {
            println!("Could not fetch aircraft data: {}", e);
            return;
        }
    };

    let mut join_set = JoinSet::new();

    /* Create a dummy server that will feed us with APRS lines.
     * This will be obsolete at one point. */
    join_set.spawn(async move {
        let address = "127.0.0.1:9000";
        let line = "TODO".as_bytes().to_vec();
        let duration = Duration::from_secs(2);

        let _ = start_dummy_server(address, line, duration).await;
    });

    let (status_tx, status_rx) = mpsc::channel(32);
    let mut server_handle = match init_api_server(&"127.0.0.1:8080", status_rx).await {
        Ok(s) => s,
        Err(e) => {
            println!("Could not start API server: {}", e);
            return;
        }
    };

    join_set.spawn(async move {
        let config = ClientConfig {
            address: "127.0.0.1:9000",
            user_name: "N0SIGN",
            password: "-1",
        };

        let _ = init_aprs_client(&config, status_tx, &aircrafts).await;
    });

    while (join_set.join_next().await).is_some() {}

    while server_handle.join_next().await.is_some() {}
}
