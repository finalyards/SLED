use core::{
    fmt,
    num::NonZeroI32
};

use crate::bindings::{
    esp_err_t,
};

#[derive(Debug)]
pub struct EspErr(pub NonZeroI32 /*esp_err_t*/);

impl EspErr {
    pub(crate) fn wrap(err: esp_err_t) -> Result<(),Self> {
        match NonZeroI32::new(err) {
            None => Ok(()),
            Some(x) => Err(Self(x)),
        }
    }
}

impl fmt::Display for EspErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let ee = esp_idf_sys::EspError::from(self.0.get()).unwrap();
        write!(f, "{}", ee)
    }
}

impl core::error::Error for EspErr {}
