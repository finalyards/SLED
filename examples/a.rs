use sled::SLed;

/// Simplest possible example. Synchronous.
///
/// Use this to test that the hardware interface works.
///
fn main() {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Lights up, world!");

    #[cfg(feature = "_gpio_8")]
    const GPIO_NUM: u8 = 8;

    let led = SLed::new(GPIO_NUM)
        .unwrap_or_else(|e| panic!("Failed to initialize: {:?}", e));

    {
        led.set_rgb(0, 0, 0); // red
    }
}
