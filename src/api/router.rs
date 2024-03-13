use super::handler::default_handler;
use axum::{routing::get, Router};
use std::io::Error;
use tokio::{net::ToSocketAddrs, sync::mpsc::Receiver, task::JoinSet};

pub async fn init_api_server<'a, A: ToSocketAddrs>(
    address: &A,
    mut message_rx: Receiver<String>,
) -> Result<(), Error> {
    let mut join_set = JoinSet::new();

    let state = super::state::create_app_state();
    let update_state = state.clone();

    join_set.spawn(async move {
        while let Some(new_message) = message_rx.recv().await {
            let mut message = update_state.api_state.lock().expect("Mutex was poisoned");
            message.message = new_message;
        }
    });

    let app = Router::new()
        .route("/:latitude/:longitude", get(default_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    join_set.spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    while (join_set.join_next().await).is_some() { /* */ }

    Ok(())
}
