use once_cell::sync::Lazy;
use regex::{Captures, Regex};
use std::collections::HashMap;

use crate::{ddb::AircraftId, time::get_current_timestamp};

use super::status::{Aircraft, Position, Status};

/// Regex pattern to extract data from valid APRS messages
///
/// # Notes
///
/// At the part of "idXXYYYYYY", "XX" must not be 40 or higher!
/// This is due to the fact that this 2-digit hex number contains the tracking-information as _binary_ in the
/// form of _0bSTxxxxxx_ and if _S_ = _1_ or _T_ = _1_, we should discard the message.
/// So all "allowed" values are in the range of _0b00000000_ - _0b00111111_, or in hex: _0x00_ - _0x3f_,
/// therefore we can discard all messages not in this range.
/// <seealso href="https://github.com/dbursem/ogn-client-php/blob/master/lib/OGNClient.php#L87"/>
const LINE_PATTERN: &str = r"h(?<latitude>[0-9.]+[NS])/(?<longitude>[0-9.]+[WE]).*?(?<course>\d{3})/(?<speed>\d{3})/A=(?<altitude>\d+).*?id[0-3]{1}[A-Fa-f0-9]{1}(?<id>[A-Za-z0-9]+).*?(?<verticalSpeed>[-0-9]+)fpm.*?(?<turnRate>[-.0-9]+)rot";

/// Factor to convert knots to km/h
const FACTOR_KNOTS_TO_KM_H: f32 = 1.852;
/// Factor to convert ft to m
const FACTOR_FT_TO_M: f32 = 0.3048;
/// Factor to convert ft/min to m/s
const FACTOR_FT_MIN_TO_M_SEC: f32 = 0.00508;
/// Factor to convert "turns/2min" to "turns/min"
const FACTOR_TURNS_TWO_MIN_TO_TURNS_MIN: f32 = 0.5;

// TODO: Use once_cell?
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
/// use aprs::Aircraft;
/// use ddb::AircraftId;
/// use std::collections::HashMap;
///
/// /* see: http://wiki.glidernet.org/wiki:ogn-flavoured-aprs */
/// let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0ADDE626 -019fpm +0.0rot 5.5dB 3e -4.3kHz";
/// let aircrafts: HashMap<AircraftId, Aircraft> = HashMap::new();
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

    let time_stamp = get_current_timestamp();

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
        time_stamp: get_current_timestamp(),
    };

    Some(status)
}

/// Tries converting a `Captures` value to `f32` and multiply it to a `conversion_factor`
///
/// # Arguments
///
/// * `captures` - The regex `Captures` to look up
/// * `name` - Name of the captured value that should be converted
/// * `conversion_factor` - The factor that the value should be multipled with
///
/// # Examples
///
/// ```
/// // We assume that `captures` will include a _turnRate_.
/// let captures = LINE_REGEX.captures(line).unwrap();
/// let turn_rate = capture_as_f32(&captures, "turnRate", FACTOR_TURNS_TWO_MIN_TO_TURNS_MIN).unwrap();
/// ```
fn capture_as_f32(captures: &Captures, name: &str, conversion_factor: f32) -> Option<f32> {
    let string_value = captures.name(name)?.as_str();
    let value = string_value.parse::<f32>().ok()?;

    Some(value * conversion_factor)
}

/// Tries converting a `Captures` value to `u16` and multiply it to a `conversion_factor`
///
/// # Arguments
///
/// * `captures` - The regex `Captures` to look up
/// * `name` - Name of the captured value that should be converted
/// * `conversion_factor` - The factor that the value should be multipled with
///
/// # Examples
///
/// ```
/// // We assume that `captures` will include a _speed_.
/// let captures = LINE_REGEX.captures(line).unwrap();
/// let speed = capture_as_u16(&captures, "speed", FACTOR_KNOTS_TO_KM_H).unwrap();
/// ```
/// # Notes
///
/// Returns `None` if _value_ * `conversion_factor` would under- / overlow `u16` ranges
fn capture_as_u16(captures: &Captures, name: &str, conversion_factor: f32) -> Option<u16> {
    let string_value = captures.name(name)?.as_str();
    let value = string_value.parse::<u16>().ok()?;
    let converted_value = value as f32 * conversion_factor;

    if converted_value < u16::MIN as f32 || converted_value > u16::MAX as f32 {
        return None;
    }

    Some(converted_value as u16)
}

/// Tries converting a `Captures` APRS coordinate value to a latitude / longitude value as `f32`
///
/// # Arguments
///
/// * `captures` - The regex `Captures` to look up
/// * `name` - Name of the captured value that should be converted
///
/// # Examples
///
/// ```
/// // We assume that `captures` will include a _latitude_ with a value of "5111.32N".
/// let captures = LINE_REGEX.captures(line).unwrap();
/// let latitude = capture_as_coordinate_value(&captures, "latitude");
///
/// print!("{}", latitude); // 51.188667
/// ```
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
