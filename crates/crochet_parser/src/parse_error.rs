use error_stack::Context;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "ParseError")
    }
}

impl Context for ParseError {}
