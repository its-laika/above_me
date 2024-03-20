use api::{init_api_server, AppState};
use aprs::init_aprs_client;
use config::load_config;
use ddb::fetch_aircrafts;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinSet,
};

mod api;
mod aprs;
mod config;
mod ddb;
mod haversine;
mod time;

#[tokio::main]
async fn main() {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            println!("Could not load config: {}", e);
            return;
        }
    };

    let aircrafts = match fetch_aircrafts(&config.ddb_url).await {
        Ok(a) => a,
        Err(e) => {
            println!("Could not fetch aircraft data: {}", e);
            return;
        }
    };

    let mut join_set = JoinSet::new();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (status_tx, mut status_rx) = mpsc::channel(32);

    let app_state = AppState::create();
    let update_state = app_state.clone();

    join_set.spawn(async move {
        init_api_server(&config.bind_to, app_state, shutdown_rx)
            .await
            .expect("Could not start API server");
    });

    join_set.spawn(async move {
        if let Err(e) = init_aprs_client(&config.aprs, status_tx, &aircrafts).await {
            println!("Client stopped with error: {}", e);
        }

        shutdown_tx.send(()).unwrap();
    });

    join_set.spawn(async move {
        while let Some(status) = status_rx.recv().await {
            update_state.push_status(status);
        }
    });

    while (join_set.join_next().await).is_some() {}
}
