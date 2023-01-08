use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

struct GeneralError {
    name: String,
    message: String,
}

impl Debug for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{{ GeneralError: name={:?} message={:?} }}", self.name, self.message))
    }
}

impl Error for GeneralError {}