
use embassy_time::{Duration, Instant};

use sled::{
    /*animations::*/ PingAnimation,
    Rgb,
};

/// A state tied to real time clock; reset by pressing the 'BOOT' button.
pub struct ButtonPing {
    /// Time of starting the current ping.
    since: Instant
}

impl ButtonPing {
    pub const fn new() -> Self {
        Self{
            since: Instant::MIN
        }
    }
}

impl PingAnimation /*<{Rgb::WHITE}>*/ for ButtonPing {
    const CYCLE: Duration = Duration::from_millis(3_000);
    const COLOR: Rgb = Rgb::RED;

    fn on_reset(&mut self) {
        self.since = Instant::now()
    }
}
