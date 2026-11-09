use embassy_time::Duration;

use sled::{
    /*animations::*/ PingAnimation,
    Rgb,
};

/// A fast-fading ping; reset by pressing the 'BOOT' button.
pub struct ButtonPing {}

impl PingAnimation for ButtonPing {
    const CYCLE: Duration = Duration::from_millis(3_000);
    const COLOR: Rgb = Rgb::RED;
}
