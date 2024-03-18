use super::state::AppState;
use crate::aprs::Status;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::io::Error;
use tokio::{net::ToSocketAddrs, sync::oneshot};

pub async fn init_api_server<'a, A: ToSocketAddrs>(
    address: &A,
    app_state: AppState,
    shutdown_rx: oneshot::Receiver<()>,
) -> Result<(), Error> {
    let app = Router::new()
        .route("/r/:latitude/:longitude/:range", get(handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await?;

    Ok(())
}

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

#[derive(Serialize)]
struct ResponseDto {
    latitude: f32,
    longitude: f32,
    range: u32,
    states: Vec<Status>,
}
