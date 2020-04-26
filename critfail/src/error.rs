use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
/// Represents an error parsing a roll expression.
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub(crate) fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string(),
        }
    }
}

impl Default for ParseError {
    fn default() -> Self {
        ParseError { details: "".into() }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
