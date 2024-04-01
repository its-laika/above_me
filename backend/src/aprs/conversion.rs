use std::collections::HashMap;

use log::debug;
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::{
    ogn::{Aircraft, AircraftId, AircraftType},
    position::Position,
    time::get_current_timestamp,
};

use super::status::Status;

/// Regex pattern to extract data from valid APRS messages
///
/// # Notes
///
/// At the part of "idXXYYYYYY", "XX" must not be 40 or higher!
/// This is due to the fact that this 2-digit hex number contains the tracking-information as
/// _binary_ in the form of _0bSTxxxxxx_ and if _S_ = _1_ or _T_ = _1_, we should discard the
/// message. So all "allowed" values are in the range of _0b00000000_ - _0b00111111_, or in hex:
/// _0x00_ - _0x3f_, therefore we can discard all messages not in this range.
///
/// see: [dbursem/ogn-client-php](https://github.com/dbursem/ogn-client-php/blob/master/lib/OGNClient.php#L87)
const LINE_PATTERN: &str = r"h(?<latitude>[0-9.]+[NS])[/\\]?.(?<longitude>[0-9.]+[WE]).(?:(?<course>\d{3})/(?<speed>\d{3})/A=(?<altitude>\d+))?.*?id(?<type>[0-3]{1}[A-Fa-f0-9]{1})(?<id>[A-Za-z0-9]+)(?: (?<verticalSpeed>[-+0-9]+)fpm)?(?: (?<turnRate>[-+.0-9]+)rot)?";

/// Factor to convert knots to km/h
const FACTOR_KNOTS_TO_KM_H: f32 = 1.852;
/// Factor to convert ft to m
const FACTOR_FT_TO_M: f32 = 0.3048;
/// Factor to convert ft/min to m/s
const FACTOR_FT_MIN_TO_M_SEC: f32 = 0.00508;
/// Factor to convert "turns/2min" to "turns/min"
const FACTOR_TURNS_TWO_MIN_TO_TURNS_MIN: f32 = 0.5;
/// Placeholder for unknown aicraft values
const UNKNOWN: &str = "";

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(LINE_PATTERN).unwrap());

/// Tries converting an APRS line into a `Status`
///
/// # Arguments
///
/// * `line` - The APRS line of the APRS server
/// * `aircraft` - Mapping of `AircraftId` => `Aircraft`, necessary for conversion
///
/// # Examples
///
/// ```
/// use ogn::{Aircraft, AircraftId}
/// use std::collections::HashMap;
///
/// let valid_aircraft = Aircraft {
///     id: String::from("AB1234"),
///     call_sign: String::from("G1"),
///     registration: String::from("D-6507"),
///     aircraft_type: String::from("ASK-21"),
///     visible: true,
/// };
///
/// let mapping = HashMap::from([(valid_aircraft.id.clone(), valid_aircraft.clone())]);
///
/// let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0AAB1234 -019fpm +0.0rot 5.5dB 3e -4.3kHz";
///
/// let result = convert(line, &mapping);
/// assert!(result.is_some());
/// assert_eq!(result.unwrap().aircraft.id, valid_aircraft.id);
/// ```
pub fn convert(line: &str, aircraft: &HashMap<AircraftId, Aircraft>) -> Option<Status> {
    let Some(captures) = LINE_REGEX.captures(line) else {
        debug!("Line not parseable");
        return None;
    };

    let id = captures.name("id")?.as_str();

    let aircraft = if let Some(a) = aircraft.get(id) {
        if a.has_model() {
            a.clone()
        } else {
            let model = get_aircraft_type_by_capture(&captures, "type")
                .map(|t| String::from(t.get_name()))
                .unwrap_or_default();

            a.with_model(model)
        }
    } else {
        debug!("Unknown aircraft id '{id}'");

        let model = get_aircraft_type_by_capture(&captures, "type")
            .map(|t| String::from(t.get_name()))
            .unwrap_or_default();

        Aircraft {
            id: String::from(id),
            call_sign: String::from(UNKNOWN),
            registration: String::from(UNKNOWN),
            model,
            visible: true,
        }
    };

    let status = Status {
        aircraft,
        position: Position {
            latitude: capture_as_coordinate_value(&captures, "latitude")?,
            longitude: capture_as_coordinate_value(&captures, "longitude")?,
        },
        speed: capture_as_u16(&captures, "speed", FACTOR_KNOTS_TO_KM_H),
        vertical_speed: capture_as_f32(&captures, "verticalSpeed", FACTOR_FT_MIN_TO_M_SEC),
        altitude: capture_as_u16(&captures, "altitude", FACTOR_FT_TO_M),
        turn_rate: capture_as_f32(&captures, "turnRate", FACTOR_TURNS_TWO_MIN_TO_TURNS_MIN),
        course: capture_as_u16(&captures, "course", 1.0),
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
/// * `conversion_factor` - The factor that the value should be multiplied with
///
/// # Examples
///
/// ```
/// let captures = Regex::new(r"(?<value>[\d.]+)")
///     .unwrap()
///     .captures("12.34")
///     .unwrap();
///
/// assert!(capture_as_f32(&captures, "value", 1.0).is_some_and(|f| f == 12.34));
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
/// * `conversion_factor` - The factor that the value should be multiplied with
///
/// # Examples
///
/// ```
/// let captures = Regex::new(r"(?<value>\d+)")
///     .unwrap()
///     .captures("1234")
///     .unwrap();
///
/// assert!(capture_as_u16(&captures, "value", 1.0).is_some_and(|f| f == 1234));
/// ```
/// # Notes
///
/// Returns `None` if _value_ * `conversion_factor` would under- / overflow `u16` ranges
fn capture_as_u16(captures: &Captures, name: &str, conversion_factor: f32) -> Option<u16> {
    let string_value = captures.name(name)?.as_str();
    let value = string_value.parse::<u16>().ok()?;
    let converted_value = f32::from(value) * conversion_factor;

    if converted_value < f32::from(u16::MIN) || converted_value > f32::from(u16::MAX) {
        return None;
    }

    /* We check for range and also sign. */
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
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
/// let captures = Regex::new(r"(?<value>.+)")
///     .unwrap()
///     .captures("1029.35S")
///     .unwrap();
///
/// assert!(capture_as_coordinate_value(&captures, "value").is_some_and(|f| f == -10.489166));
/// ```
fn capture_as_coordinate_value(captures: &Captures, name: &str) -> Option<f32> {
    /* Latitude and longitude (by APRS-standard) are given as following: ddmm.mmD where d = "degree",
     * m = "minute" and D = "direction".
     * Notice that minutes are decimals, so 0.5 minutes equal 0 minutes, 30 secs.
     * We'll separate degrees and minutes, so we can convert it to a "degree"-only value. */

    let string_value = captures.name(name)?.as_str();
    let aprs_value = string_value
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<f32>()
        .ok()?;

    let orientation = string_value.chars().last()?; /* "N", "E", "S" or "W" */

    let degrees = f32::floor(aprs_value / 1_0000.0); // Separating   "dd" from "ddmmmm"
    let minutes = f32::floor(aprs_value % 1_0000.0) // Separating "mmmm" from "ddmmmm"
        / 60.0 // because 60 minutes = 1 degree
        / 100.0; // because of the removed decimal separator

    if (f64::from(degrees) + f64::from(minutes)) > f64::from(f32::MAX) {
        /* Don't think that's possible but just to be sure */
        return None;
    }

    let value = degrees + minutes;

    if orientation == 'S' || orientation == 'W' {
        Some(value * -1.0)
    } else {
        Some(value)
    }
}

/// Tries extracting the aircraft type from a `Capture` of the first encoded id field
///
/// # Arguments
///
/// * `captures` - The regex `Captures` to look up
/// * `name` - Name of the captured value that should be converted
///
/// # Examples
///
/// ```
/// let capture = Regex::new(r"(?<value>.*)")
///     .unwrap()
///     .captures("2D")
///     .unwrap();
///
/// let aircraft_type = et_aircraft_type_by_capture(&capture, "value");
///
/// assert!(aircraft_typ.is_some());
/// assert!(aircraft_type.unwrap() == AircraftType::Balloon);
/// ```
///
/// # References
///
/// - [OGN Wiki](http://wiki.glidernet.org/wiki:ogn-flavoured-aprs#toc2)
fn get_aircraft_type_by_capture(captures: &Captures, name: &str) -> Option<AircraftType> {
    let string_value = captures.name(name)?.as_str();
    let value = u8::from_str_radix(string_value, 16).ok()?;

    /* Aircraft id field is built like this: "id" + "XX" + "YYYYYY" where the last part is the
     * aircraft identifier (that matches DDB). "XX" is a 2-digit hex value built like this:
     * 0bSTttttaa where "S" indicates stealth mode (should *NEVER* be sent over OGN), "T" is
     * no-tracking mode (should *NEVER* be parsed by above_me), "tttt" is the aircraft type
     * and "aa" is the address type. As we only need "tttt", we shift the whole number two
     * digits to the left and just care for the lowest four bytes. */

    if value & 0b1000_0000 > 0 {
        /* This should *NEVER* happen! */
        panic!("Line to be parsed has stealth mode active");
    }

    if value & 0b0100_0000 > 0 {
        /* This should *NEVER* happen! */
        panic!("Line to be parsed has no-tracking mode active");
    }

    let aircraft_type_value = (value >> 2) & 0b1111;
    AircraftType::from_aprs_u8(aircraft_type_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_type_works() {
        let regex = Regex::new(r"(?<value>.*)").unwrap();

        assert!(
            get_aircraft_type_by_capture(&regex.captures("2D").unwrap(), "value")
                .is_some_and(|t| t == AircraftType::Balloon)
        );

        assert!(
            get_aircraft_type_by_capture(&regex.captures("07").unwrap(), "value")
                .is_some_and(|t| t == AircraftType::Glider)
        );

        assert!(get_aircraft_type_by_capture(&regex.captures("00").unwrap(), "XXX").is_none());
        assert!(get_aircraft_type_by_capture(&regex.captures("ZZZZ").unwrap(), "value").is_none());
        assert!(get_aircraft_type_by_capture(&regex.captures("00").unwrap(), "value").is_none());
    }

    #[test]
    #[should_panic]
    fn get_type_panics_on_stealth_mode() {
        let regex = Regex::new(r"(?<value>.*)").unwrap();
        get_aircraft_type_by_capture(&regex.captures("80").unwrap(), "value");
    }

    #[test]
    #[should_panic]
    fn get_type_panics_on_no_tracking_mode() {
        let regex = Regex::new(r"(?<value>.*)").unwrap();
        get_aircraft_type_by_capture(&regex.captures("40").unwrap(), "value");
    }

    #[test]
    fn convert_works() {
        let valid_aircraft = Aircraft {
            id: String::from("AB1234"),
            call_sign: String::from("G1"),
            registration: String::from("D-6507"),
            model: String::from("ASK-21"),
            visible: true,
        };

        let mapping = HashMap::from([(valid_aircraft.id.clone(), valid_aircraft.clone())]);

        let data_set = &[
            (
                "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0AAB1234 -019fpm +0.0rot 5.5dB 3e -4.3kHz",
                valid_aircraft.id.as_str(),
                51.188667,
                Some(12),
                Some(-0.09652),
                Some(185),
                Some(0.0),
                Some(86)
            ),
            (
                "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N\\00102.04W'086/007/A=000607 id0AAB1234 5.5dB 3e -4.3kHz",
                valid_aircraft.id.as_str(),
                51.188667,
                Some(12),
                None,
                Some(185),
                None,
                Some(86)
            ),
            (
                "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N\\00102.04W' id0AAB1234 5.5dB 3e -4.3kHz",
                valid_aircraft.id.as_str(),
                51.188667,
                None,
                None,
                None,
                None,
                None
            ),
            (
                "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W' id0AAB1234 -019fpm +0.0rot 5.5dB 3e -4.3kHz",
                valid_aircraft.id.as_str(),
                51.188667,
                None,
                Some(-0.09652),
                None,
                Some(0.0),
                None
            ),
            (
                "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0AAB1234 +0.0rot 5.5dB 3e -4.3kHz",
                valid_aircraft.id.as_str(),
                51.188667,
                Some(12),
                None,
                Some(185),
                Some(0.0),
                Some(86)
            ),
            (
                "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W' id0AAB1234 +0.0rot 5.5dB 3e -4.3kHz",
                valid_aircraft.id.as_str(),
                51.188667,
                None,
                None,
                None,
                Some(0.0),
                None
            ),
        ];

        for (line, aircraft_id, latitude, speed, vertical_speed, altitude, turn_rate, course) in
            data_set
        {
            let result = convert(line, &mapping);
            assert!(result.is_some());

            let status = result.unwrap();
            assert_eq!(&status.aircraft.id, aircraft_id);
            assert_eq!(&status.position.latitude, latitude);
            assert_eq!(&status.speed, speed);
            assert_eq!(&status.vertical_speed, vertical_speed);
            assert_eq!(&status.altitude, altitude);
            assert_eq!(&status.turn_rate, turn_rate);
            assert_eq!(&status.course, course);
            assert!(status.time_stamp > 0);
        }
    }

    #[test]
    fn convert_works_with_unknown_aircraft() {
        let mapping = HashMap::new();

        let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id0AAB1234 -019fpm +0.0rot 5.5dB 3e -4.3kHz";

        let result = convert(line, &mapping);
        assert!(result.is_some());

        let status = result.unwrap();
        assert_eq!(status.aircraft.id, "AB1234");
        assert_eq!(status.aircraft.call_sign, "");
        assert_eq!(status.aircraft.registration, "");
        assert_eq!(status.aircraft.model, "Tow plane");
        assert!(status.aircraft.visible);
    }

    #[test]
    fn convert_ignores_stealth_mode() {
        let mapping = HashMap::new();
        let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id8AAB1234 -019fpm +0.0rot 5.5dB 3e -4.3kHz";
        assert!(convert(line, &mapping).is_none());
    }

    #[test]
    fn convert_ignores_no_tracking_mode() {
        let mapping = HashMap::new();
        let line = "FLRDDE626>APRS,qAS,EGHL:/074548h5111.32N/00102.04W'086/007/A=000607 id4AAB1234 -019fpm +0.0rot 5.5dB 3e -4.3kHz";
        assert!(convert(line, &mapping).is_none());
    }

    #[test]
    fn test_capture_as_f32_works() {
        let captures = Regex::new(r"(?<value>[\d.]+)")
            .unwrap()
            .captures("12.34")
            .unwrap();

        assert!(capture_as_f32(&captures, "value", 1.0).is_some_and(|f| f == 12.34));
        assert!(capture_as_f32(&captures, "value", 2.0).is_some_and(|f| f == 24.68));
    }

    #[test]
    fn test_capture_as_u16_works() {
        let captures = Regex::new(r"(?<value>\d+)")
            .unwrap()
            .captures("1234")
            .unwrap();

        assert!(capture_as_u16(&captures, "value", 1.0).is_some_and(|f| f == 1234));
        assert!(capture_as_u16(&captures, "value", 2.0).is_some_and(|f| f == 2468));
    }

    #[test]
    fn test_capture_as_u16_fails_on_out_of_range() {
        let captures = Regex::new(r"(?<value>\d+)")
            .unwrap()
            .captures("1234")
            .unwrap();

        assert!(capture_as_u16(&captures, "value", 10.0).is_some());
        assert!(capture_as_u16(&captures, "value", 100.0).is_none());
        assert!(capture_as_u16(&captures, "value", -1.0).is_none());
    }

    #[test]
    fn test_capture_as_coordinate_value_works() {
        let captures = Regex::new(r"(?<value>.+)")
            .unwrap()
            .captures("5111.32N")
            .unwrap();

        assert!(capture_as_coordinate_value(&captures, "value").is_some_and(|f| f == 51.188667));
    }

    #[test]
    fn test_capture_as_coordinate_value_works_on_negative() {
        let captures = Regex::new(r"(?<value>.+)")
            .unwrap()
            .captures("1029.35S")
            .unwrap();

        assert!(capture_as_coordinate_value(&captures, "value").is_some_and(|f| f == -10.489166));
    }
}
