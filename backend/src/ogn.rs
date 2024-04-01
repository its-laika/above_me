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

impl Aircraft {
    /// Returns whether `Aircraft` has a defined model
    ///
    /// # Examples
    ///
    /// ```
    /// let with_model = Aircraft {
    ///     id: String::from("AB1234"),
    ///     call_sign: String::from("G1"),
    ///     registration: String::from("D-6507"),
    ///     model: String::from("ASK-21"),
    ///     visible: true,
    /// };
    ///
    /// let without_model = Aircraft {
    ///     id: String::from("AB1234"),
    ///     call_sign: String::from("G1"),
    ///     registration: String::from("D-6507"),
    ///     model: String::from(""),
    ///     visible: true,
    /// };
    ///
    /// assert!(with_model.has_model());
    /// assert!(!without_model.has_model());
    /// ```
    pub fn has_model(&self) -> bool {
        !self.model.is_empty()
    }

    /// Clones `Aircraft` with a given `model` name
    ///
    /// # Arguments
    /// * `model` - The new model name that the resulting aircraft
    ///    should have
    ///
    /// # Examples
    ///
    /// ```
    /// let aircraft = Aircraft {
    ///     id: String::from("AB1234"),
    ///     call_sign: String::from("G1"),
    ///     registration: String::from("D-6507"),
    ///     model: String::from(""),
    ///     visible: true,
    /// };
    ///
    /// let aircraft_with_model = aircraft.with_model(String::new("ASK-21"));
    ///
    /// assert!(aircraft_with_model.has_model());
    /// assert_eq!(aircraft_with_model.model, "ASK-21");
    /// ```
    pub fn with_model(&self, model: String) -> Aircraft {
        Aircraft {
            id: self.id.clone(),
            call_sign: self.call_sign.clone(),
            registration: self.registration.clone(),
            model,
            visible: self.visible,
        }
    }
}

/// Representation of generic aicraft types.
#[derive(PartialEq)]
pub enum AircraftType {
    Glider,
    Tow,
    Helicopter,
    SkyDiver,
    DropPlane,
    HangGlider,
    Paraglider,
    MotorAircraft,
    Jet,
    Balloon,
    Blimp,
    Unmanned,
    Obstacle,
}

impl AircraftType {
    /// Tries getting aircaft type for the APRS aircraft type value
    /// (encoded inside the aircraft id field).
    ///
    /// # Arguments
    ///
    /// * `id` - Aircraft type id
    ///
    /// # Examples
    /// ```
    /// assert_eq!(AircraftType::from_aprs_u8(15), AircraftType::Obstacle);
    /// assert_eq!(AircraftType::from_aprs_u8(0), None);
    /// ```
    ///
    /// # References
    /// - `aprs::get_aircraft_type_by_capture`
    /// - [OGN Wiki](http://wiki.glidernet.org/wiki:ogn-flavoured-aprs#toc2)
    pub fn from_aprs_u8(id: u8) -> Option<AircraftType> {
        match id {
            1 => Some(Self::Glider),
            2 => Some(Self::Tow),
            3 => Some(Self::Helicopter),
            4 => Some(Self::SkyDiver),
            5 => Some(Self::DropPlane),
            6 => Some(Self::HangGlider),
            7 => Some(Self::Paraglider),
            8 => Some(Self::MotorAircraft),
            9 => Some(Self::Jet),
            11 => Some(Self::Balloon),
            12 => Some(Self::Blimp),
            13 => Some(Self::Unmanned),
            15 => Some(Self::Obstacle),
            _ => None,
        }
    }

    /// Returns the (english) name of an `AircraftType`
    ///
    /// # Examples
    /// ```
    /// assert_eq!(AircraftType::Glider.get_name(), "(Motor) Glider");
    /// ```
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Glider => "(Motor) Glider",
            Self::Tow => "Tow plane",
            Self::Helicopter => "Helicopter / Gyrocopter",
            Self::SkyDiver => "Skydiver / Parachute",
            Self::DropPlane => "Drop plane",
            Self::HangGlider => "Hang glider",
            Self::Paraglider => "Paraglider",
            Self::MotorAircraft => "Motor aircaft",
            Self::Jet => "Jet",
            Self::Balloon => "Balloon",
            Self::Blimp => "Blimp",
            Self::Unmanned => "Unmanned (Drone)",
            Self::Obstacle => "Obstacle",
        }
    }
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
