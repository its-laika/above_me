use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    Json,
};

use super::{state::AppState, MAX_AGE_DIFF, POSITION_RADIUS};
use crate::{aprs::Status, mutex::get_locked};
use serde::Serialize;

/// Handler for the "default" route that returns a list of aircraft for a given position
pub async fn default_handler(
    Path((latitude, longitude)): Path<(f32, f32)>,
    State(api_state): State<AppState>,
) -> Json<ResponseDto> {
    let states = get_locked(&api_state.states)
        .values()
        .filter(|&s| filter_status(s, latitude, longitude))
        .cloned()
        .collect::<Vec<Status>>();

    Json(ResponseDto {
        latitude,
        longitude,
        states,
    })
}

#[derive(Serialize)]
pub struct ResponseDto {
    latitude: f32,
    longitude: f32,
    states: Vec<Status>,
}

/// Returns whether to include a given `Status` in an API response
///
/// # Arguments
///
/// * `status` - The status that may be included
/// * `latitude` - Provided latitude that the aircraft should be near of
/// * `longitude` - Provided longitude that the aircraft should be near of
fn filter_status(status: &Status, latitude: f32, longitude: f32) -> bool {
    if f32::abs(status.position.latitude - latitude) > POSITION_RADIUS {
        return false;
    }
    if f32::abs(status.position.longitude - longitude) > POSITION_RADIUS {
        return false;
    }

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Could not get unix timestamp")
        .as_secs();

    if current_timestamp - status.time_stamp > MAX_AGE_DIFF {
        return false;
    }

    true
}
