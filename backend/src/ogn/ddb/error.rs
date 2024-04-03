use std::fmt::{Display, Formatter, Result};

/// Enum of `Error`s for failing HTTP requests
#[derive(Debug)]
pub enum Http {
    /// Could not fetch data
    FetchError,
    /// The response is not valid
    ResponseError,
}

impl Display for Http {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::FetchError => write!(f, "Could not fetch data"),
            Self::ResponseError => write!(f, "Invalid response"),
        }
    }
}
