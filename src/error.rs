use std::error;
use std::fmt;
use std::io;

// https://stackoverflow.com/a/31749071 - Macros within crates
// These are inspired by various third-party crate libs

/// Exits a function early with an `Error`.
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err(crate::ErrorWrapper::new($e));
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(crate::ErrorWrapper::new(format!($fmt, $($arg)*)));
    };
}

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// Similar to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`,
/// it does not panic.
#[macro_export]
macro_rules! ensure {
    ($cond:expr) => {
        if !($cond) {
            bail!("{}", stringify!($cond));
        }
    };
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}

#[derive(Debug)]
pub enum ErrorWrapper {
    IOError(io::Error),
    Wrapped(Box<dyn error::Error>),
    ParsingError(String),
    Simple(String),
    Numbered(i64),
    NotImplemented,
}
impl ErrorWrapper {
    pub fn new<S: Into<String>>(msg: S) -> ErrorWrapper {
        ErrorWrapper::Simple(msg.into())
    }
}
impl From<io::Error> for ErrorWrapper {
    fn from(e: io::Error) -> Self {
        ErrorWrapper::IOError(e)
    }
}
impl From<std::num::ParseIntError> for ErrorWrapper {
    fn from(e: std::num::ParseIntError) -> Self {
        ErrorWrapper::ParsingError(format!("{}", e))
    }
}
impl From<std::num::ParseFloatError> for ErrorWrapper {
    fn from(e: std::num::ParseFloatError) -> Self {
        ErrorWrapper::ParsingError(format!("{}", e))
    }
}
impl From<Box<dyn error::Error>> for ErrorWrapper {
    fn from(e: Box<dyn error::Error>) -> Self {
        ErrorWrapper::Wrapped(e)
    }
}
impl From<&str> for ErrorWrapper {
    fn from(e: &str) -> Self {
        ErrorWrapper::Simple(e.to_string())
    }
}
#[allow(unreachable_patterns)]
impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorWrapper::Simple(v) => write!(f, "Error: {}", v),
            ErrorWrapper::Numbered(v) => write!(f, "Error: {}", v),
            ErrorWrapper::Wrapped(v) => write!(f, "Error: {:?}", v),
            ErrorWrapper::IOError(v) => write!(f, "Error: {:?}", v),
            ErrorWrapper::ParsingError(v) => write!(f, "Error: {}", v),
            ErrorWrapper::NotImplemented => write!(f, "Not implemented"),
            _ => write!(f, "{:?}", self)
        }
    }
}
impl Into<String> for ErrorWrapper {
    fn into(self) -> String {
        format!("{}", self)
    }
}
impl error::Error for ErrorWrapper {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self {
            ErrorWrapper::IOError(e) => Some(e),
            ErrorWrapper::Wrapped(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
