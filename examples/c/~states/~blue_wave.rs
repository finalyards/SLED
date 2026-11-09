
use embassy_time::{Duration, Instant};

use sled::{
    animator::PingAnimator,
    Rgb,
};

/// A non-state that simply waves a blue, peaceful light...
pub struct BlueWave {
    // no state
}

impl BlueWave {
    pub const fn new() -> Self {
        Self {}
    }
}

impl RepeatingAnimator /*<{Rgb::WHITE}>*/ for BlueWave {
    const CYCLE: Duration = Duration::from_millis(5_000);
    //const REPEAT: Repeat = Repeat::BackAndForth;

    const START: Rgb = Rgb::BLUE;
    const END: Rgb = Rgb::NONE;
}
