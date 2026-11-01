
use esp_idf_svc::{
    sys::link_patches,
};

use embassy_executor::Spawner;

use embassy_time::{
    Duration,
    Instant,
};

use sled::{
    animator::PulseAnimator,
    Animator,
    Repeat,
    Rgb,
    SLed
};

//const BLUE: Rgb = Rgb { r: 0, g: 0, b: 100 };
//const NONE: Rgb = Rgb { r: 0, g: 0, b: 0 };

/// Asynchronous example.
///
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Lights up, world!");

    let st1 = PulsingBlue::new();
    let st2 = ButtonPing::new();

    let led = SLed::new([&st1, &st2])
        .unwrap_or_else(|e| panic!("Failed to initialize: {:?}", e));

    loop {
        led.set_rgb(255, 0, 0) .unwrap(); // R
        embassy_time::Timer::after_secs(1).await;

        led.set_rgb(0, 255, 0) .unwrap(); // G
        embassy_time::Timer::after_secs(1).await;

        led.set_rgb(0, 0, 255) .unwrap(); // B
        embassy_time::Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn sled_task() -> ! {
    let sled = SLed::new()
        .unwrap_or_else(|e| panic!("Failed to initialize the SLed: {:?}", e));

    sled.run_loop() .await;
}

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
    const REPEAT: Repeat = Repeat::BackAndForth;

    //const MODE: PingAnimatorMode = Linear;    // tbd. Linear | Exponential
}
