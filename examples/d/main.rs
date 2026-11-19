use esp_idf_svc::{
    sys::link_patches,
};

use embassy_executor::Spawner;
use embassy_time::Duration;

use sled::{
    anims::PingAnimation,
    Animation,
    Rgb,
    SLed
};

mod state_anim;
use state_anim::StateAnim;

enum States {
    A,
    B,
    C,
}

static STATE_ANIM: StateAnim = StateAnim::new(
    // Map of
    A -> PulsingRed,
    B -> StableBlue,
    C -> RampGreen,
);

const ANIMS: &[&dyn sled::Animation] = &[&STATE_ANIM];

/// Example with:
///     - animation that changes by the application's state
///
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Log level defined by 'RUST_LOG' env.var.
    esp_idf_svc::log::init_from_env();

    log::info!("Lights up, world!");

    // Run the animations
    spawner.spawn(
        sled_task() .unwrap_or_else(|e| panic!("spawn failed: {:?}", e))
    );

    // Application runs elsewhere; provides its states.
    loop {
        // Pseudo-random next state; random delay, too.

        let next_st = match nearly_random(3) {
            0 => State::A,
            1 => State::B,
            2 => State::C,
        };

        let delay = Duration::from_millis(1000 + nearly_random(4000));

        STATE_ANIM.show(next_st);
        embassy_time::Timer::after(delay).await;
    }
}

#[embassy_executor::task]
async fn sled_task() -> ! {
    let led = SLed::new(ANIMS)
        .unwrap_or_else(|e| panic!("Failed to initialize: {:?}", e));

    led.run_loop() .await;
}

/// A simple way to get random-ish numbers, without pulling in a dependency.
fn random(max: u32) -> u32 {
    unsafe { esp_idf_sys::esp_rand() } % max
}

struct PulsingRed;

impl Animation for PulsingRed {
    fn sample
}

B -> StableBlue,
C -> RampGreen,
