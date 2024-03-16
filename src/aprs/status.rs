use serde::Serialize;

/// Representation of an aircraft status
#[derive(Clone, Serialize)]
pub struct Status {
    pub aircraft: Aircraft,
    pub position: Position,
    pub speed: u16,
    pub vertical_speed: f32,
    pub altitude: u16,
    pub turn_rate: f32,
    pub course: u16,
    pub time_stamp: u64,
}

/// Representing information about an aircraft.
/// Note that aircrafts, that shouldn't be published, will be filtered out before,
/// so that a "hide flag" isn't necessary
#[derive(Clone, Serialize)]
pub struct Aircraft {
    pub id: String,
    pub call_sign: String,
    pub registration: String,
    pub aircraft_type: String,
}

/// Representation of a position
#[derive(Clone, Serialize)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
}
