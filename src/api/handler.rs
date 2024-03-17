use axum::{
    extract::{Path, State},
    Json,
};

use super::state::{AppState, MAX_AGE_DIFF};
use crate::{aprs::Status, time::get_current_timestamp};
use serde::Serialize;

/* approximated. (meaning: copied from the internet.) */
const FACTOR_LATITUDE_KM_TO_DEG: f32 = 0.00905;
const FACTOR_LONGITUDE_KM_TO_DEG: f32 = 0.000905;

/// Handler for the "default" route that returns a list of aircraft for a given position
pub async fn default_handler(
    Path((latitude, longitude, range)): Path<(f32, f32, f32)>,
    State(app_state): State<AppState>,
) -> Json<ResponseDto> {
    let states = app_state
        .get_states()
        .values()
        .filter(|&s| filter_status(s, latitude, longitude, range))
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
/// * `range` - Approx. range in kilometers that should be regarded
fn filter_status(status: &Status, latitude: f32, longitude: f32, range: f32) -> bool {
    if f32::abs(status.position.latitude - latitude) > FACTOR_LATITUDE_KM_TO_DEG * range {
        return false;
    }

    if f32::abs(status.position.longitude - longitude) > FACTOR_LONGITUDE_KM_TO_DEG * range {
        return false;
    }

    if get_current_timestamp() - status.time_stamp > MAX_AGE_DIFF {
        return false;
    }

    true
}
