use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum AppError {
    MissingData,
    TimezoneConstruct,
    MiddayConstruct,
    TimestampParse,
}

#[derive(Debug)]
pub enum Error {
    App(AppError),
    Log(log::SetLoggerError),
    Io(io::Error),
    Yaml(serde_yaml::Error),
    Json(serde_json::Error),
    Url(url::ParseError),
    Websocket(tungstenite::Error),
    Spa(sunrise_sunset_calculator::spa::SpaError),
}

impl error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::MissingData => write!(f, "Missing data key in json response"),
            AppError::TimezoneConstruct => write!(f, "Cannot construct FixedOffset timezone"),
            AppError::MiddayConstruct => write!(f, "Cannot construct midday DateTime"),
            AppError::TimestampParse => write!(f, "Cannot construct NaiveDateTime from timestamp"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::App(_) => None,
            Error::Log(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Yaml(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::Url(e) => Some(e),
            Error::Websocket(e) => Some(e),
            Error::Spa(e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::App(e) => e.fmt(f),
            Error::Log(e) => e.fmt(f),
            Error::Io(e) => e.fmt(f),
            Error::Yaml(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
            Error::Url(e) => e.fmt(f),
            Error::Websocket(e) => e.fmt(f),
            Error::Spa(e) => e.fmt(f),
        }
    }
}

impl From<AppError> for Error {
    fn from(error: AppError) -> Self {
        Error::App(error)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(error: log::SetLoggerError) -> Self {
        Error::Log(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::Yaml(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::Url(error)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(error: tungstenite::Error) -> Self {
        Error::Websocket(error)
    }
}

impl From<sunrise_sunset_calculator::spa::SpaError> for Error {
    fn from(error: sunrise_sunset_calculator::spa::SpaError) -> Self {
        Error::Spa(error)
    }
}
