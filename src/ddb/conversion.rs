use crate::aprs::Aircraft;

const VALUE_YES: &str = "Y";
const FIELD_SEPARATOR: char = ',';
const IDENTIFIER_COMMENT: char = '#';

const INDEX_ID: usize = 1;
const INDEX_TYPE: usize = 2;
const INDEX_REGISTRATION: usize = 3;
const INDEX_CALL_SIGN: usize = 4;
const INDEX_TRACKED: usize = 5;
const INDEX_IDENTIFIED: usize = 6;

/// Tries converting a line of DDB into an `Aircraft` representation
///
/// # Arguments
///
/// * `line` - The line that should be converted
///
/// # Examples
///
/// ```
/// let line = "ABC123,ASK-21,D-6507,G1,Y,Y";
/// let aircraft = convert(line).unwrap();
///
/// print!("Callsign: {}", aircraft.call_sign); // "Callsign: G1"
/// ```
pub fn convert(line: &str) -> Option<Aircraft> {
    if line.starts_with(IDENTIFIER_COMMENT) {
        return None;
    }

    let fields = line
        .split(FIELD_SEPARATOR)
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    if fields.len() < 7 {
        return None;
    }

    let id = fields[INDEX_ID].to_string();
    if id.is_empty() {
        return None;
    }

    let call_sign = fields[INDEX_CALL_SIGN].to_string();
    let registration = fields[INDEX_REGISTRATION].to_string();
    let aircraft_type = fields[INDEX_TYPE].to_string();
    let visible = fields[INDEX_IDENTIFIED] == VALUE_YES && fields[INDEX_TRACKED] == VALUE_YES;

    Some(Aircraft {
        id,
        call_sign,
        registration,
        aircraft_type,
        visible,
    })
}
