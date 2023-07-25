



use std::fmt;

use std::sync::Mutex;

use std::sync::atomic::{AtomicBool, Ordering};







// // Error handling
pub enum MatricalError {
    Regular(MatricalErrorType),
    Custom(String),
    InvalidValue,
    InvalidContext,
    ShouldNotOccur,
    IndexOutOfBounds,
}

pub enum AtomicBoolError {
    MutexPoisoned,
    IndexOutOfBounds,
    MissingOperand,
}

pub enum MatricalErrorType {
    IncorrectDimensions,
    IncorrectFormat,
}

impl fmt::Debug for MatricalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



impl fmt::Display for MatricalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatricalError::Regular(err) => write!(f, "Regular error: {}", err.as_str()),
            MatricalError::Custom(err) => write!(f, "Custom error: {}", err),
            MatricalError::ShouldNotOccur => write!(f, "Other error"),
            MatricalError::InvalidValue => write!(f, "Invalid value"),
            MatricalError::InvalidContext => write!(f, "Invalid context"),  
            MatricalError::IndexOutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}

impl MatricalErrorType {
    fn as_str(&self) -> &str {
        match *self {
            MatricalErrorType::IncorrectDimensions => "IncorrectDimensions",
            MatricalErrorType::IncorrectFormat => "IncorrectFormat",
        }
    }
}

// ERROR

#[derive(Debug)]
pub struct Error {
    message: String,
    // you can add more fields here
}

// impl Error {
//     pub fn new(message: &str) -> Self {
//         Error {
//             message: message.to_string(),
//         }
//     }
// }