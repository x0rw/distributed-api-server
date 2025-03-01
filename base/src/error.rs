use std::fmt;
use std::io;
pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    Custom(String),
    InvalidHttpReqSize,
    UnknowenHttpMethod,
    NullHeaderReq,
    IO(io::Error),
    InvalidHeader,
    EmptyHeaderField,
    CloseConnection,
}
impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::custom(val.to_string())
    }
}
impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
