use std::io::Error;

use axum::{routing::get, Router};
use log::info;
use tokio::{net::TcpListener, net::ToSocketAddrs, sync::oneshot};

use super::routes::{aircraft, overview};
use super::state::App;

/// Initializes a tcp server that serves our API
///
/// # Arguments
///
/// * `address` - The address that the server will bind to
/// * `app` - The `App` that the API will use for its data
/// * `shutdown_rx` - A oneshot `Receiver<()>` that will shut down the server gracefully when a
///    message is received.
///
/// # Returns
///
/// Future that will either result to () or Error when an error occurs.
///
/// # Examples
///
/// ```
/// use api::App;
/// use tokio::{spawn, sync::oneshot};
///
/// let address = "127.0.0.1:8080";
/// let (shutdown_tx, shutdown_rx) = oneshot::channel();
/// let app = App::create();
///
/// spawn(async move {
///     api::init(&address, app, shutdown_rx)
///         .await
///         .expect("API server failed");
/// });
///
/// // Shuts down API server
/// shutdown_tx.send(()).unwrap();
/// ```
pub async fn init<'a, A: ToSocketAddrs>(
    address: &A,
    app: App,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/r/:latitude/:longitude/:range", get(aircraft::handler))
        .route("/status", get(overview::handler))
        .with_state(app);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
            info!("API received shutdown signal");
        })
        .await?;

    Ok(())
}
