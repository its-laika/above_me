use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::ddb::AircraftId;

use super::status::{Aircraft, Position, Status};

const LINE_PATTERN: &str = r"h(?<latitude>[0-9.]+[NS])/(?<longitude>[0-9.]+[WE]).*?(?<course>\d{3})/(?<speed>\d{3})/A=(?<altitude>\d+).*?id[0-3]{1}[A-Fa-f0-9]{1}(?<id>[A-Za-z0-9]+).*?(?<verticalSpeed>[-0-9]+)fpm.*?(?<turnRate>[-.0-9]+)rot";

/// Factor to convert knots to km/h
const FACTOR_KNOTS_TO_KM_H: f32 = 1.852;
/// Factor to convert ft to m
const FACTOR_FT_TO_M: f32 = 0.3048;
/// Factor to convert ft/min to m/s
const FACTOR_FT_MIN_TO_M_SEC: f32 = 0.00508;
/// Factor to convert "turns/2min" to "turns/min"
const FACTOR_TURNS_TWO_MIN_TO_TURNS_MIN: f32 = 0.5;

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(LINE_PATTERN).unwrap());

/// Tries converting an APRS line into a `Status`
///
/// # Arguments
///
/// * `line` - The APRS line of the APRS server
/// * `aircrafts` - Mapping of `AircraftId` => `Aircraft`, necessary for conversion
///
/// # Examples
///
/// ```
/// /* see: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs */
/// let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0ADDE626 -019fpm +0.0rot 5.5dB 3e -4.3kHz";
/// let aircrafts: std::collections::HashMap<ddb::AircraftId, aprs::Aircraft> = std::collections::HashMap::new();
///
/// let status = convert(line).await;
///
/// print!("Callsign: {}", status.aircraft.callsign); // "Callsign: ABCDE"
/// ```
pub async fn convert(line: &str, aircrafts: &HashMap<AircraftId, Aircraft>) -> Option<Status> {
    let captures = match LINE_REGEX.captures(line) {
        Some(c) => c,
        None => return None,
    };

    let id = captures.name("id")?.as_str();

    let aircraft = match aircrafts.get(id) {
        Some(a) => a.clone(),
        None => return None,
    };

    let time_stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Could not get unix timestamp")
        .as_secs();

    let status = Status {
        aircraft,
        position: Position {
            latitude: capture_as_coordinate_value(&captures, "latitude")?,
            longitude: capture_as_coordinate_value(&captures, "longitude")?,
        },
        speed: capture_as_u16(&captures, "speed", FACTOR_KNOTS_TO_KM_H)?,
        vertical_speed: capture_as_f32(&captures, "verticalSpeed", FACTOR_FT_MIN_TO_M_SEC)?,
        altitude: capture_as_u16(&captures, "altitude", FACTOR_FT_TO_M)?,
        turn_rate: capture_as_f32(&captures, "turnRate", FACTOR_TURNS_TWO_MIN_TO_TURNS_MIN)?,
        course: capture_as_u16(&captures, "course", 1.0)?,
        time_stamp,
    };

    Some(status)
}

fn capture_as_f32(captures: &Captures, name: &str, conversion_factor: f32) -> Option<f32> {
    let string_value = captures.name(name)?.as_str();
    let value = string_value.parse::<f32>().ok()?;

    Some(value * conversion_factor)
}

fn capture_as_u16(captures: &Captures, name: &str, conversion_factor: f32) -> Option<u16> {
    let string_value = captures.name(name)?.as_str();
    let value = string_value.parse::<u16>().ok()?;
    let converted_value = value as f32 * conversion_factor;

    if converted_value < u16::MIN as f32 || converted_value > u16::MAX as f32 {
        return None;
    }

    Some(converted_value as u16)
}

fn capture_as_coordinate_value(captures: &Captures, name: &str) -> Option<f32> {
    /* Latitude and longitude (by APRS-standard) are given as following: ddmm.mmD where d = "degree",
     * m = "minute" and D = "direction".
     * Notice that minutes are decimals, so 0.5 minutes equal 0 minutes, 30 secs.
     * We'll separate degrees and minutes, so we can convert it to a "degree"-only value. */

    let string_value = captures.name(name)?.as_str();
    let aprs_value = string_value
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u32>()
        .ok()?;

    let orientation = string_value.chars().last()?; /* "N", "E", "S" or "W" */

    let degrees = aprs_value / 1_0000; // Separating   "dd" from "ddmmmm"
    let minutes = (aprs_value % 1_0000) as f32 // Separating "mmmm" from "ddmmmm"
                      / 60.0 // because 60 minutes = 1 degree
                      / 100.0; // because of the removed decimal separator

    if (degrees as f64 + minutes as f64) > f32::MAX as f64 {
        /* Don't think that's possible but just to be sure */
        return None;
    }

    let value = degrees as f32 + minutes;

    if orientation == 'S' || orientation == 'W' {
        Some(value * -1.0)
    } else {
        Some(value)
    }
}
