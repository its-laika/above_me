use super::state::App;
use crate::{aprs::Status, position::Position};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::io::Error;
use tokio::{net::TcpListener, net::ToSocketAddrs, sync::oneshot};

/// Initializes a tcp server that serves our API
///
/// # Arguments
///
/// * `address` - The address that the server will bind to
/// * `app` - The `App` that the API will use for its data
/// * `shutdown_rx` - A oneshot `Receiver<()>` that will shutdown the server gracefully when a message is received.
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
        .route("/r/:latitude/:longitude/:range", get(handler))
        .with_state(app);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await?;

    Ok(())
}

/// Default handler for our (only) API route
async fn handler(
    Path((latitude, longitude, range)): Path<(f32, f32, f32)>,
    State(app): State<App>,
) -> Json<ResponseDto> {
    /* Ensure range can be used as f32 */
    let position = Position {
        latitude,
        longitude,
    };

    Json(ResponseDto {
        latitude,
        longitude,
        range,
        states: app.get_filtered_states(&position, range),
    })
}

/// Response DTO for our (only) API route
/// see [openapi.yml](../../openapi.yml)
#[derive(Serialize)]
struct ResponseDto {
    /// Equals given latitude parameter
    latitude: f32,
    /// Equals given longitude parameter
    longitude: f32,
    /// Equals given range parameter
    range: f32,
    /// The aircraft states that match the given parameters
    states: Vec<Status>,
}
