use std::error;
use std::fmt;
use std::num;

#[macro_export]
macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::convert::From::from($expr));
    )
}

pub struct ParseError {
    repr: ErrorRepr,
}

// This is all types of errors that can happen.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ErrorKind {
    UnknownCommand,
    MalformedCommand
}

// Having a special case for with detail means I can pass back dynamic data such
// as what it couldn't parse.
#[derive(Debug)]
enum ErrorRepr {
    WithDescription(ErrorKind, &'static str),
    WithDescriptionAndDetail(ErrorKind, &'static str, String)
}

impl ParseError {

    /// Returns the kind of the error.
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            ErrorRepr::WithDescription(kind, _) => kind,
            ErrorRepr::WithDescriptionAndDetail(kind, _, _) => kind
        }
    }
}

impl PartialEq for ParseError {
    fn eq(&self, other: &ParseError) -> bool {
        match (&self.repr, &other.repr) {
            (&ErrorRepr::WithDescription(kind_a, _),
             &ErrorRepr::WithDescription(kind_b, _)) => {
                kind_a == kind_b
            }
            (&ErrorRepr::WithDescriptionAndDetail(kind_a, _, _),
             &ErrorRepr::WithDescriptionAndDetail(kind_b, _, _)) => {
                kind_a == kind_b
            },
            _ => false,
        }
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(ref err: num::ParseIntError) -> ParseError {
        ParseError{repr: ErrorRepr::WithDescriptionAndDetail(
            ErrorKind::MalformedCommand,
            "failed to parse as an integer",
            error::Error::description(err).to_owned()
        )}
    }
}

impl From<(ErrorKind, &'static str)> for ParseError {
    fn from((kind, description): (ErrorKind, &'static str)) -> ParseError {
        ParseError{repr: ErrorRepr::WithDescription(kind, description)}
    }
}

impl From<(ErrorKind, &'static str, String)> for ParseError {
    fn from((kind, description, detail): (ErrorKind, &'static str, String)) -> ParseError {
        ParseError{repr: ErrorRepr::WithDescriptionAndDetail(kind, description, detail)}
    }
}

impl error::Error for ParseError {
    fn description(&self) -> &str {
        match self.repr {
            ErrorRepr::WithDescription(_, description) => description,
            ErrorRepr::WithDescriptionAndDetail(_, description, _) => description,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.repr {
            ErrorRepr::WithDescription(_, description) => {
                description.fmt(f)
            }
            ErrorRepr::WithDescriptionAndDetail(_, description, ref detail) => {
                try!(description.fmt(f));
                try!(f.write_str(": "));
                detail.fmt(f)
            }
        }
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self, f)
    }
}
