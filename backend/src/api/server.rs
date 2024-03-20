use super::state::AppState;
use crate::aprs::Status;
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
/// * `app_state` - The `AppState` that the API will use for its data
/// * `shutdown_rx` - A oneshot `Receiver<()>` that will shutdown the server gracefully when a message is received.
///
/// # Returns
///
/// Future that will either result to () or Error when an error occurs.
///
/// # Examples
///
/// ```
/// use api::AppState;
/// use tokio::{spawn, sync::oneshot};
///
/// let address = "127.0.0.1:8080";
/// let (shutdown_tx, shutdown_rx) = oneshot::channel();
/// let app_state = AppState::create();
///
/// spawn(async move {
///     init_api_server(&address, app_state, shutdown_rx)
///         .await
///         .expect("API server failed");
/// });
///
/// // Shuts down API server
/// shutdown_tx.send(()).unwrap();
/// ```
pub async fn init_api_server<'a, A: ToSocketAddrs>(
    address: &A,
    app_state: AppState,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/r/:latitude/:longitude/:range", get(handler))
        .with_state(app_state);

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
    Path((latitude, longitude, range)): Path<(f32, f32, u32)>,
    State(app_state): State<AppState>,
) -> Json<ResponseDto> {
    /* Ensure range can be used as f32 */
    let range: u32 = if range > f32::MAX as u32 {
        f32::MAX as u32
    } else {
        range
    };

    Json(ResponseDto {
        latitude,
        longitude,
        range,
        states: app_state.get_filtered_states(latitude, longitude, range as f32),
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
    range: u32,
    /// The aircraft states that match the given parameters
    states: Vec<Status>,
}
