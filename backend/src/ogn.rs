use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

/// Representing information about an aircraft.
#[derive(Clone, Serialize)]
pub struct Aircraft {
    /// DDB id of the aircraft
    #[serde(skip_serializing)]
    pub id: String,
    /// Call sign, e.g. "G1"
    pub call_sign: String,
    /// Registration, e.g. "D-6507"
    pub registration: String,
    /// Aircraft model type, e.g. "ASK-21"
    pub model: String,
    /// Should the aircraft be identified and tracked?
    #[serde(skip_serializing)]
    pub visible: bool,
}

/// Alias for `String`, just for readability.
pub type AircraftId = String;

impl Display for Aircraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "[ Id: {}, Call sign: {}, Registration: {}, Type: {}, Visible: {} ]",
            self.id, self.call_sign, self.registration, self.model, self.visible
        )
    }
}
