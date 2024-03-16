use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum HttpError {
    FetchError,
    ResponseError,
}

impl Display for HttpError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::FetchError => write!(f, "Could not fetch data"),
            Self::ResponseError => write!(f, "Invalid response"),
        }
    }
}
