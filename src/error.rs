use std::fmt::Display;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

pub use crate::audio::error::AudioError;

#[derive(Debug, Clone, AsRefStr)]
pub enum Error {
    Audio(AudioError)
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}