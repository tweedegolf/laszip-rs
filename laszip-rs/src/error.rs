use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, LaszipError>;

#[derive(Debug)]
pub struct LaszipError {
    pub error: String,
}

impl fmt::Display for LaszipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for LaszipError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
