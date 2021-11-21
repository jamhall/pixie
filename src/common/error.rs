use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    Configuration(String),
    InvalidCommand(String),
    Transport(String),
    Display,
    IoError(std::io::Error),
}

impl Error for ApplicationError {}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::Configuration(message) => write!(formatter, "{}", message),
            ApplicationError::Display => write!(formatter, "Display error"),
            ApplicationError::InvalidCommand(message) => write!(formatter, "Invalid command: {}", message),
            ApplicationError::IoError(err) => {
                writeln!(formatter, "IoError: {}", err)
            }
            ApplicationError::Transport(message) => write!(formatter, "{}", message)
        }
    }
}

impl From<clap::Error> for ApplicationError {
    fn from(error: clap::Error) -> Self {
        ApplicationError::Configuration(error.to_string())
    }
}

impl From<std::io::Error> for ApplicationError {
    fn from(err: std::io::Error) -> Self {
        ApplicationError::IoError(err)
    }
}
