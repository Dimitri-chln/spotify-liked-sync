use std::env::VarError;
use std::fmt::Display;

use crate::Result;

#[derive(Debug)]
pub enum Error {
    // Internal errors
    InvalidQuery,
    NoAuthCode,

    // External errors
    Spotify(spotify_rs::Error),
    Io(std::io::Error),
    Var(VarError),
    Serde(serde_json::Error),
    Unknown(Box<dyn std::error::Error + Send + Sync>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidQuery => write!(f, "Invalid query received in URL"),
            Error::NoAuthCode => write!(f, "No auth code or CSRF state was found in URL"),
            Error::Spotify(error) => error.fmt(f),
            Error::Io(error) => error.fmt(f),
            Error::Var(error) => error.fmt(f),
            Error::Serde(error) => error.fmt(f),
            Error::Unknown(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<spotify_rs::Error> for Error {
    fn from(value: spotify_rs::Error) -> Self {
        Self::Spotify(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<VarError> for Error {
    fn from(value: VarError) -> Self {
        Self::Var(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

pub trait Transform<T> {
    fn transform(self) -> Result<T>;
}

impl<T, E> Transform<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn transform(self) -> Result<T> {
        self.map_err(|err| Error::Unknown(Box::new(err)))
    }
}
