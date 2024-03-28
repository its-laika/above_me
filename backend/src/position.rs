use std::fmt::{Display, Formatter, Result};

use serde::Serialize;

/// Representation of a position
#[derive(Clone, Serialize)]
pub struct Position {
    /// Latitude
    pub latitude: f32,
    /// Longitude
    pub longitude: f32,
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
