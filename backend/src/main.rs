use ddb::fetch_aircraft;
use tokio::{
    sync::{mpsc, oneshot},
    task::JoinSet,
};

mod api;
mod aprs;
mod config;
mod ddb;
mod haversine;
mod ogn;
mod position;
mod time;

#[tokio::main]
async fn main() {
    let config = match config::load() {
        Ok(c) => c,
        Err(e) => {
            println!("Could not load config: {e}");
            return;
        }
    };

    let aircraft = match fetch_aircraft(&config.ddb_url).await {
        Ok(a) => a,
        Err(e) => {
            println!("Could not fetch aircraft data: {e}");
            return;
        }
    };

    let mut join_set = JoinSet::new();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let (status_tx, mut status_rx) = mpsc::channel(32);

    let app = api::App::create();
    let app_update = app.clone();

    join_set.spawn(async move {
        api::init(&config.bind_to, app, shutdown_rx)
            .await
            .expect("Could not start API server");
    });

    join_set.spawn(async move {
        if let Err(e) = aprs::init(&config.aprs, status_tx, &aircrafts).await {
            println!("Client stopped with error: {e}");
        }

        shutdown_tx.send(()).unwrap();
    });

    join_set.spawn(async move {
        while let Some(status) = status_rx.recv().await {
            app_update.push_status(status);
        }
    });

    while (join_set.join_next().await).is_some() {}
}
