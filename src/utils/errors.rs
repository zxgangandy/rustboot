
use std::fmt::{self, Debug, Display};
use std::io;
pub type Result<T> = std::result::Result<T, Error>;

/// A generic error that represents all the ways a method can fail inside of rexpr::core.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Default Error
    E(String),
}

impl Display for Error {
    // IntellijRust does not understand that [non_exhaustive] applies only for downstream crates
    // noinspection RsMatchCheck
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(error) => write!(f, "{}", error),
        }
    }
}

impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return Error::E(arg);
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Self {
        Error::from(err.to_string())
    }
}

impl From<rocket::Error> for Error {
    #[inline]
    fn from(err: rocket::Error) -> Self {
        Error::from(err.to_string())
    }
}

