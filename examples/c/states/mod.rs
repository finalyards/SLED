

const STATES_N: usize = 2;

#[embassy_executor::task]
async fn sled_task(led: SLed::<STATES_N>) -> ! {

    led.run_loop() .await;
}


/// A state tied to real time clock; reset by pressing the 'BOOT' button.
struct ButtonPing {
    since: Instant
}

impl ButtonPing {
    fn new() -> Self { Self{
        since: Instant::MIN
    } }
}

impl PingAnimator /*<{Rgb::WHITE}>*/ for ButtonPing {
    const CYCLE: Duration = Duration::from_millis(3_000);
    const COLOR: Rgb = Rgb::RED;
}


/*** disabled for now
/// A stateless state (just for testing visuals); pulses blue light.
struct PulsingBlue {
}

impl PulsingBlue {
    fn new() -> Self { Self{} }
}

// Note: didn't -yet- get the const params to work. 'PulseAnimator' currently fixed with those values.
//
impl PulseAnimator /*<{Rgb::BLUE}, {Rgb::NONE}>*/ for PulsingBlue {
    const CYCLE: Duration = Duration::from_millis(5_000);
    const REPEAT: Repeat = Repeat::BackAndForth;
}
***/
