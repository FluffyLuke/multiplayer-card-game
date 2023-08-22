use std::{fmt};

#[derive(Debug, Clone)]
pub struct NoSettingsFileError;
impl fmt::Display for NoSettingsFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No \"settings.txt\" file found in the same directory with program.")
    }
}

#[derive(Debug, Clone)]
pub enum ServerSetupError {
    NoSettingFoundError(&'static str),
    BadPortNumberError(&'static str),
}

impl fmt::Display for ServerSetupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if NoSettingFoundError(err) = self {
            
        }
    }
}