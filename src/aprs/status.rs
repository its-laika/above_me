use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

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
#[derive(Clone, Serialize)]
pub struct Aircraft {
    pub id: String,
    pub call_sign: String,
    pub registration: String,
    pub aircraft_type: String,
    pub visible: bool,
}

/// Representation of a position
#[derive(Clone, Serialize)]
pub struct Position {
    pub latitude: f32,
    pub longitude: f32,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
