use axum::{
    extract::{Path, State},
    Json,
};

use super::state::AppState;
use serde::Serialize;

pub async fn default_handler(
    Path((latitude, longitude)): Path<(f32, f32)>,
    State(api_state): State<AppState>,
) -> Json<ResponseDto> {
    Json(ResponseDto {
        latitude,
        longitude,
        message: api_state
            .api_state
            .lock()
            .expect("Mutext was poisoned")
            .message
            .clone(),
    })
}

#[derive(Serialize)]
pub struct ResponseDto {
    latitude: f32,
    longitude: f32,
    message: String,
}
