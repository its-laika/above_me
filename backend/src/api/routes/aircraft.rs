use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::{api::App, aprs::Status, position::Position};

/// Handler for route _/r/:latitude/:longitude/:range_
///
/// Responds with a list of aircraft in the _:range_ around _:latitude_ and _:longitude_
pub async fn handler(
    Path((latitude, longitude, range)): Path<(f32, f32, f32)>,
    State(app): State<App>,
) -> Json<Response> {
    /* Ensure range can be used as f32 */
    let position = Position {
        latitude,
        longitude,
    };

    Json(Response {
        latitude,
        longitude,
        range,
        states: app.get_filtered_states(&position, range),
    })
}

#[derive(Serialize)]
pub struct Response {
    /// Equals given latitude parameter
    latitude: f32,
    /// Equals given longitude parameter
    longitude: f32,
    /// Equals given range parameter
    range: f32,
    /// The aircraft states that match the given parameters
    states: Vec<Status>,
}
