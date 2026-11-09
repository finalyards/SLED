use embassy_executor::{
    Spawner
};
use embassy_time::Duration;
use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

use esp_idf_svc::{
    sys::link_patches,
};

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
static PING_ANIMATION: PingAnimation = PingAnimation::new(Rgb::WHITE, 3_000);
static FLAT_ANIMATION: StaticColor = StaticColor {};

/// Simplest (non-)animation one can make.
///
/// Good for testing the output brightness limit
///
struct StaticColor;

impl Animation for StaticColor {
    fn sample(&self, _: Duration) -> Rgb {
        Rgb::RED
    }
}

const ANIMS: &[&dyn sled::Animation] = &[/*&PING_ANIMATION,*/ &FLAT_ANIMATION];

/// Example with:
///     - multiple states
///     - interactive state change
///
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Log level defined by 'RUST_LOG' env.var.
    esp_idf_svc::log::init_from_env();

    // tbd. Such tests will be moved away.
    //fake_test::run_all();

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

// Wasn't able to run RGB/HSV tests as (host side) unit tests. #Hack
//
// tbd. Make this a separate example ('t'), eventually runnable with emulation?
//
#[cfg(false)]
mod fake_test {
    use sled::{Hsv, Rgb};

    const EPSILON: f32 = 0.001;

    // Apufunktio HSV-arvojen vertailuun liukulukkopyöristykset huomioiden
    fn assert_hsv_near(actual: Hsv, expected_h_deg: f32, expected_s: f32, expected_v: f32) {
        // Jos kyseessä on musta tai täysin harmaa, sävyllä (H) ei ole merkitystä,
        // joten tarkistetaan se vain jos saturaatio tai kirkkaus on merkittävä.
        if expected_v > EPSILON && expected_s > EPSILON {
            assert_h_near(actual, expected_h_deg);
        }
        assert_sv_near(actual, expected_s, expected_v);
    }
    fn assert_h_near(actual: Hsv, expected_h_deg: f32) {
        assert!((actual.h_deg - expected_h_deg).abs() < 1.0 /*deg*/, "Hue is off: {} vs. {}", actual.h_deg, expected_h_deg);
    }
    fn assert_sv_near(actual: Hsv, expected_s: f32, expected_v: f32) {
        assert!((actual.s - expected_s).abs() < EPSILON, "Saturation is off: {} vs. {}", actual.s, expected_s);
        assert!((actual.v - expected_v).abs() < EPSILON, "Value is off: {} vs. {}", actual.v, expected_v);
    }

    pub(crate) fn run_all() {
        test_rgb_to_hsv_primary_colors();
        test_rgb_to_hsv_black_and_white();
        test_hsv_to_rgb_roundtrip();
    }

    fn test_rgb_to_hsv_primary_colors() {
        // R: RGB(255, 0, 0) -> HSV(0.0, 1.0, 1.0)
        let red_hsv = Hsv::from(Rgb { r: 255, g: 0, b: 0 });
        assert_hsv_near(red_hsv, 0.0, 1.0, 1.0);

        // G: RGB(0, 255, 0) -> HSV(120.0, 1.0, 1.0)
        let green_hsv = Hsv::from(Rgb { r: 0, g: 255, b: 0 });
        assert_hsv_near(green_hsv, 120.0, 1.0, 1.0);

        // B: RGB(0, 0, 255) -> HSV(240.0, 1.0, 1.0)
        let blue_hsv = Hsv::from(Rgb { r: 0, g: 0, b: 255 });
        assert_hsv_near(blue_hsv, 240.0, 1.0, 1.0);
    }

    fn test_rgb_to_hsv_black_and_white() {
        // Black (LED off): RGB(0, 0, 0) -> V 0.0 (S,V ignored)
        let black_hsv = Hsv::from(Rgb { r: 0, g: 0, b: 0 });
        assert_h_near(black_hsv, 0.0);

        // White: RGB(255, 255, 255) -> Saturation 0.0, Value 1.0 (H ignored)
        let white_hsv = Hsv::from(Rgb { r: 255, g: 255, b: 255 });
        assert_sv_near(white_hsv, 0.0, 1.0);
    }

    fn test_hsv_to_rgb_roundtrip() {
        // Testataan, että muunnos HSV -> RGB palauttaa alkuperäiset arvot
        let targets = vec![
            Rgb { r: 255, g: 0, b: 0 },   // Red
            Rgb { r: 0, g: 255, b: 0 },   // Green
            Rgb { r: 0, g: 0, b: 255 },   // Blue
            Rgb { r: 128, g: 128, b: 128 }, // Gray
            Rgb { r: 255, g: 128, b: 0 },   // Orange
        ];

        for original_rgb in targets {
            let hsv = Hsv::from(original_rgb);
            let roundtrip_rgb = Rgb::from(hsv);
            assert_eq!(original_rgb, roundtrip_rgb, "Roundtrip failed: {:?} -> {:?} -> {:?}", original_rgb, hsv, roundtrip_rgb);
        }
    }
}
