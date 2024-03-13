use std::time::Duration;

use aprs::{init_aprs_client, ClientConfig};
use server::start_dummy_server;
use tokio::{sync::mpsc, task::JoinSet};

mod api;
mod aprs;
mod server;

#[tokio::main]
async fn main() {
    let mut join_set = JoinSet::new();

    /* Create a dummy server that will feed us with messages.
     * This will be obsolete at one point. */
    join_set.spawn(async move {
        let address = "127.0.0.1:9000";
        let message = "Message".as_bytes().to_vec();
        let duration = Duration::from_secs(2);

        let _ = start_dummy_server(address, message, duration).await;
    });

    let (message_tx, message_rx) = mpsc::channel(32);

    join_set.spawn(async move {
        let config = ClientConfig {
            address: "127.0.0.1:9000",
            user_name: "N0SIGN",
            password: "-1",
        };

        let _ = init_aprs_client(&config, message_tx).await;
    });

    join_set.spawn(async move {
        let _ = api::router::init_api_server(&"127.0.0.1:8080", message_rx).await;
    });

    while (join_set.join_next().await).is_some() { /* */ }
}
