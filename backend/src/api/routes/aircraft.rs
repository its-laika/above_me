use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;

use crate::{
    api::App,
    ogn::{aprs::Status, Aircraft},
    position::Position,
};

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
        states: app.get_filtered_status_dtos(&position, range),
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
    states: Vec<StatusDto>,
}

/// Dto representation of an aircraft status, containing the distance to the
/// requested postion in km.
#[derive(Clone, Serialize)]
pub struct StatusDto {
    /// Affected aircraft
    pub aircraft: Aircraft,
    /// Position of aircraft
    pub position: Position,
    /// Speed in _km/h_
    pub speed: Option<u16>,
    /// Vertical speed in _m/s_
    pub vertical_speed: Option<f32>,
    /// Altitude in _m_
    pub altitude: Option<u16>,
    /// Turn rate in _turns/min_
    pub turn_rate: Option<f32>,
    /// Course of aircraft
    pub course: Option<u16>,
    /// Timestamp of receiving status
    pub time_stamp: u64,
    /// Distance to given postion in km
    pub distance: f32,
}

impl StatusDto {
    pub fn from(status: &Status, distance: f32) -> Self {
        Self {
            aircraft: status.aircraft.clone(),
            position: status.position.clone(),
            speed: status.speed,
            vertical_speed: status.vertical_speed,
            altitude: status.altitude,
            turn_rate: status.turn_rate,
            course: status.course,
            time_stamp: status.time_stamp,
            distance,
        }
    }
}
