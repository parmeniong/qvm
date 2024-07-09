use ansiterm::Color;
use std::fmt::{Display, Formatter};

pub enum ErrorType {
    HttpError,
    NetworkError,
    VersionError
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ErrorType::HttpError => write!(f, "HTTPError"),
            ErrorType::NetworkError => write!(f, "NetworkError"),
            ErrorType::VersionError => write!(f, "VersionError")
        }
    }
}

pub fn error<T>(error_type: ErrorType, message: T)
where
    T: Display
{
    eprintln!("{} {}: {}", Color::Red.bold().paint("error"), error_type, message);
}