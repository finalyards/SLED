
use esp_idf_svc::{
    sys::link_patches,
};

use embassy_executor::Spawner;
use sled::SLed;

/// Asynchronous example.
///
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Lights up, world!");

    #[cfg(feature = "_gpio_8")]
    const GPIO_NUM: u8 = 8;

    let led = SLed::new(GPIO_NUM)
        .unwrap_or_else(|e| panic!("Failed to initialize: {:?}", e));

    loop {
        led.set_rgb(255, 0, 0); // R
        embassy_time::Timer::after_secs(1).await;

        led.set_rgb(0, 255, 0); // G
        embassy_time::Timer::after_secs(1).await;

        led.set_rgb(0, 0, 255); // B
        embassy_time::Timer::after_secs(1).await;
    }
}
