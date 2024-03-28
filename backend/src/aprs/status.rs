use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

use crate::{ogn::Aircraft, position::Position};

/// Representation of an aircraft status
#[derive(Clone, Serialize)]
pub struct Status {
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
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ 
    Aircraft: {},
    Position: {},
    Speed: {:?},
    Vertial speed: {:?},
    Altitude: {:?},
    Turn rate: {:?},
    Course: {:?},
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
