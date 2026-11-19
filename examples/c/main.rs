
use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

use esp_idf_svc::{
    sys::link_patches,
};

use embassy_executor::Spawner;
use embassy_time::Duration;

mod button_task;
use button_task::button_task;
use sled::{
    anims::PingAnimation,
    Animation,
    Rgb,
    SLed
};

//static BLUE_WAVE_ANIMATION: BlueWave = BlueWave::new();

/// Indicates press of the 'BOOT' button.
static PING_ANIMATION: PingAnimation = PingAnimation::new(Rgb::BLUE, 1_500);
static FLAT_ANIMATION: StaticColor = StaticColor {};

/// Simplest (non-)animation one can make.
///
/// Good for testing the output brightness limit
///
struct StaticColor;

impl Animation for StaticColor {
    fn sample(&self, _: Duration) -> Rgb {
        //Rgb::RED.with_value(0.5)    // tbd. allow better use of both HSV and RGB
        Rgb { r: 64, g: 0, b: 0 }   // dimmer red
    }
}

const ANIMS: &[&dyn sled::Animation] = &[&PING_ANIMATION, &FLAT_ANIMATION];

/// Example with:
///     - dual, overlapping animations (color mix)
///     - interactive state change (by BOOT button)
///
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Log level defined by 'RUST_LOG' env.var.
    esp_idf_svc::log::init_from_env();

    log::info!("Lights up, world!");

    let peripherals = Peripherals::take().unwrap();
    {
        // Note: The BOOT button is intentionally not having its own feature. It's an application
        //      side detail and we don't want to burden the library's 'Cargo.toml' too much.
        //
        let boot_pin = if cfg!(any(feature = "espressif_devkit_m1", feature = "espressif_devkit_c1")) {
            peripherals.pins.gpio9
        } else {
            panic!("Unable to pick a pin for the BOOT button; please enable a devkit feature.")
        };

        // Set 'BOOT' btn as input (pressed is Low)
        let btn_pin = PinDriver::input(boot_pin, Pull::Up).unwrap();

        spawner.spawn( button_task(btn_pin, &PING_ANIMATION).unwrap() );
    }

    // Run the animations
    spawner.spawn(
        sled_task() .unwrap_or_else(|e| panic!("spawn failed: {:?}", e))
    );

    // Everything happens now in the background.
    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn sled_task() -> ! {
    let led = SLed::new(ANIMS)
        .unwrap_or_else(|e| panic!("Failed to initialize: {:?}", e));

    led.run_loop() .await;
}
