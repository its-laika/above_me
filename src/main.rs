use std::time::Duration;

use client::init_tcp_client;
use server::start_dummy_server;
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
    time::interval,
};

mod client;
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

    let address = "127.0.0.1:9000";
    let (keep_alive_tx, keep_alive_rx) = watch::channel(());
    let (message_tx, mut message_rx) = mpsc::channel(32);

    join_set.spawn(async move {
        let _ = init_tcp_client(address, keep_alive_rx, message_tx).await;
    });

    /* Make sure to trigger a keep alive to the server every 10 minutes to not lose connection. */
    join_set.spawn(async move {
        let mut interval = interval(Duration::from_secs(60 * 10));

        loop {
            interval.tick().await;
            keep_alive_tx.send(()).unwrap();
        }
    });

    join_set.spawn(async move {
        while let Some(message) = message_rx.recv().await {
            println!("Client got message: '{}'", message);
        }
    });

    while (join_set.join_next().await).is_some() { /* */ }
}
