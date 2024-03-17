use axum::{
    extract::{Path, State},
    Json,
};

use super::state::AppState;
use crate::aprs::Status;
use serde::Serialize;

/// Handler for the "default" route that returns a list of aircraft for a given position
pub async fn default_handler(
    Path((latitude, longitude, range)): Path<(f32, f32, f32)>,
    State(app_state): State<AppState>,
) -> Json<ResponseDto> {
    Json(ResponseDto {
        latitude,
        longitude,
        states: app_state.get_filtered_states(latitude, longitude, range),
    })
}

#[derive(Serialize)]
pub struct ResponseDto {
    latitude: f32,
    longitude: f32,
    states: Vec<Status>,
}
