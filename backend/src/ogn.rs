use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

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

/// Alias for `String`, just for readability.
pub type AircraftId = String;

impl Display for Aircraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ Id: {}, Callsign: {}, Registration: {}, Type: {}, Visible: {} ]",
            self.id, self.call_sign, self.registration, self.aircraft_type, self.visible
        )
    }
}
