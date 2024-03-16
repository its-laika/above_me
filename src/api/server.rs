use crate::aprs::Status;

use super::{handler::default_handler, state::update_app_state};
use axum::{routing::get, Router};
use std::io::Error;
use tokio::{
    net::ToSocketAddrs,
    sync::{mpsc, oneshot},
    task::JoinSet,
};

pub async fn init_api_server<'a, A: ToSocketAddrs>(
    address: &A,
    mut status_rx: mpsc::Receiver<Status>,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<JoinSet<()>, Error> {
    let mut join_set = JoinSet::new();

    let state = super::state::create_app_state();
    let update_state = state.clone();

    join_set.spawn(async move {
        while let Some(status) = status_rx.recv().await {
            update_app_state(status, &update_state).await;
        }
    });

    let app = Router::new()
        .route("/:latitude/:longitude", get(default_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    join_set.spawn(async move {
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async {
                shutdown_rx.await.ok();
            })
            .await;
    });

    Ok(join_set)
}
