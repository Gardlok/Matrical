



use std::fmt;

use std::sync::Mutex;

use std::sync::atomic::{AtomicBool, Ordering};







// // Error handling
#[derive(Debug)]
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

#[derive(Debug)]
pub enum MatricalErrorType {
    IncorrectDimensions,
    IncorrectFormat,
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

#[cfg(test)]
mod tests {
    use super::{MatricalError, MatricalErrorType};

    #[test]
    fn debug_formatting_identifies_regular_error_types() {
        let cases = [
            (
                MatricalErrorType::IncorrectDimensions,
                "IncorrectDimensions",
            ),
            (MatricalErrorType::IncorrectFormat, "IncorrectFormat"),
        ];

        for (error_type, expected) in cases {
            let rendered = format!("{:?}", MatricalError::Regular(error_type));

            assert!(rendered.contains("Regular"));
            assert!(rendered.contains(expected));
        }
    }

    #[test]
    fn debug_formatting_identifies_non_regular_variants() {
        let cases = [
            (
                MatricalError::Custom("r1c custom context".to_string()),
                "Custom",
                Some("r1c custom context"),
            ),
            (MatricalError::InvalidValue, "InvalidValue", None),
            (MatricalError::InvalidContext, "InvalidContext", None),
            (MatricalError::ShouldNotOccur, "ShouldNotOccur", None),
            (MatricalError::IndexOutOfBounds, "IndexOutOfBounds", None),
        ];

        for (error, expected_variant, expected_context) in cases {
            let rendered = format!("{:?}", error);

            assert!(rendered.contains(expected_variant));

            if let Some(expected_context) = expected_context {
                assert!(rendered.contains(expected_context));
            }
        }
    }
}
