use std::error;

#[derive(Debug, Clone)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<String>,
    {
        Self { msg: msg.into() }
    }
}

impl error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.msg)
    }
}
