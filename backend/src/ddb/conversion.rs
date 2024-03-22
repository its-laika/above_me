use crate::ogn::Aircraft;

const VALUE_YES: &str = "Y";
const FIELD_SEPARATOR: char = ',';
const IDENTIFIER_COMMENT: char = '#';
const FIELD_ENCLOSURE: char = '\'';
const EMPTY: &str = "";

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
/// let aircraft = convert("'O','AB1234','ASK-21','D-6507','G1','Y','Y'").unwrap();
/// assert_eq!(aircraft.registration, "D-6507");
/// ```
pub fn convert(line: &str) -> Option<Aircraft> {
    if line.starts_with(IDENTIFIER_COMMENT) {
        return None;
    }

    let line = line.replace(FIELD_ENCLOSURE, EMPTY);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_correctly() {
        let line = "'O','AB1234','ASK-21','D-6507','G1','Y','Y'";

        let result = convert(line);
        assert!(result.is_some());

        let aircraft = result.unwrap();
        assert_eq!(aircraft.id, "AB1234");
        assert_eq!(aircraft.call_sign, "G1");
        assert_eq!(aircraft.registration, "D-6507");
        assert_eq!(aircraft.aircraft_type, "ASK-21");
        assert!(aircraft.visible);
    }

    #[test]
    fn sets_visible_correctly() {
        assert!(convert("'O','AB1234','','','','Y','Y'").is_some_and(|a| a.visible));
        assert!(convert("'O','AB1234','','','','Y','N'").is_some_and(|a| !a.visible));
        assert!(convert("'O','AB1234','','','','N','Y'").is_some_and(|a| !a.visible));
        assert!(convert("'O','AB1234','','','','N','N'").is_some_and(|a| !a.visible));
    }
}
