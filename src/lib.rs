#![no_std]
#![feature(adt_const_params)]
#![feature(min_adt_const_params)]
#![feature(const_param_ty_trait)]
// for using 'struct' in const generics
//? #![allow(incomplete_features)]  // needed?

mod raw;
pub use raw::SLed;

mod color;
pub use color::{Hsv, Rgb};

pub mod animator;   // 'animator::pulse' etc.
pub use animator::{Animator, Repeat};

// It's needed at the build, on the 'dependencies' side. This removes a warning.
use esp_idf_sys as _;

