
use embassy_time::{Duration, Instant};

use sled::{
    animator::{PingAnimator /*, PulseAnimator*/},
    Rgb,
};

/// A state tied to real time clock; reset by pressing the 'BOOT' button.
pub(crate) struct ButtonPing {
    since: Instant
}

impl ButtonPing {
    pub(crate) fn new() -> Self { Self{
        since: Instant::MIN
    } }
}

impl PingAnimator /*<{Rgb::WHITE}>*/ for ButtonPing {
    const CYCLE: Duration = Duration::from_millis(3_000);
    const COLOR: Rgb = Rgb::RED;
}
