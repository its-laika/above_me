use crate::aprs::Status;
use crate::mutex::get_locked;

use super::handler::default_handler;
use axum::{routing::get, Router};
use std::io::Error;
use tokio::{net::ToSocketAddrs, sync::mpsc::Receiver, task::JoinSet};

pub async fn init_api_server<'a, A: ToSocketAddrs>(
    address: &A,
    mut status_rx: Receiver<Status>,
) -> Result<JoinSet<()>, Error> {
    let mut join_set = JoinSet::new();

    let state = super::state::create_app_state();
    let update_state = state.clone();

    join_set.spawn(async move {
        while let Some(status) = status_rx.recv().await {
            get_locked(&update_state.states).insert(status.aircraft.call_sign.clone(), status);
        }
    });

    let app = Router::new()
        .route("/:latitude/:longitude", get(default_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    join_set.spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    Ok(join_set)
}
