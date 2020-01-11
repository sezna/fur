use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GopherParseError {}

impl fmt::Display for GopherParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GopherParseError is here!")
    }
}

impl Error for GopherParseError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }
}
