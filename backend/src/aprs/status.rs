use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

/// Representation of an aircraft status
#[derive(Clone, Serialize)]
pub struct Status {
    /// Affected aircraft
    pub aircraft: Aircraft,
    /// Position of aircraft
    pub position: Position,
    /// Speed in _km/h_
    pub speed: u16,
    /// Vertical speed in _m/s_
    pub vertical_speed: f32,
    /// Altitude in _m_
    pub altitude: u16,
    /// Turn rate in _turns/min_
    pub turn_rate: f32,
    /// Course of aircraft
    pub course: u16,
    /// Timestamp of receiving status
    pub time_stamp: u64,
}

/// Representing information about an aircraft.
#[derive(Clone, Serialize)]
pub struct Aircraft {
    /// DDB id of the aircraft
    pub id: String,
    /// Callsign, e.g. "G1"
    pub call_sign: String,
    /// Registration, e.g. "D-6507"
    pub registration: String,
    /// Aircraft model type, e.g. "ASK-21"
    pub aircraft_type: String,
    /// Should the aircraft be identified and tracked?
    pub visible: bool,
}

/// Representation of a position
#[derive(Clone, Serialize)]
pub struct Position {
    /// Latitude
    pub latitude: f32,
    /// Longitude
    pub longitude: f32,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ 
    Aircraft: {},
    Position: {},
    Speed: {},
    Vertial speed: {},
    Altitude: {},
    Turn rate: {},
    Course: {},
    Timestamp: {}
]",
            self.aircraft,
            self.position,
            self.speed,
            self.vertical_speed,
            self.altitude,
            self.turn_rate,
            self.course,
            self.time_stamp
        )
    }
}

impl Display for Aircraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ Id: {}, Callsign: {}, Registration: {}, Type: {}, Visible: {} ]",
            self.id, self.call_sign, self.registration, self.aircraft_type, self.visible
        )
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ Latitude: {}, Longitude: {} ]",
            self.latitude, self.longitude
        )
    }
}
