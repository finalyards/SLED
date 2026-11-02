use core::fmt;

use super::raw::EspErr;

/// Public errors provided by the library.
#[derive(Debug)]
pub struct Error(EspErr);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // tbd. consider wrapping it
        self.0.fmt(f)
    }
}

impl From<EspErr> for Error {
    fn from(err: EspErr) -> Self {
        Self(err)
    }
}

impl core::error::Error for Error {}
