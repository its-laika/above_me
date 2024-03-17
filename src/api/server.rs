use super::{handler::default_handler, state::AppState};
use axum::{routing::get, Router};
use std::io::Error;
use tokio::{net::ToSocketAddrs, sync::oneshot};

pub async fn init_api_server<'a, A: ToSocketAddrs>(
    address: &A,
    app_state: AppState,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/r/:latitude/:longitude/:range", get(default_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await?;

    Ok(())
}
