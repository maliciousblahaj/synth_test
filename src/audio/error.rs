use std::fmt::Display;

use strum_macros::AsRefStr;


#[derive(Debug, Clone, AsRefStr)]
pub enum AudioError {
    AudioGraphInvalidId(u32),
}



impl std::error::Error for AudioError {}

impl Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}