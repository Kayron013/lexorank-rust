use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError(pub String);

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
