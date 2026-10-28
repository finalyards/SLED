use esp_idf_svc;

use sled::BuiltInLed;

use embassy_executor as _;

fn main() {
    // It is necessary to call this function once.
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Lights up, world!");

    #[cfg(feature = "_gpio_8")]
    const GPIO_NUM: u8 = 8;

    if let Ok(led) = BuiltInLed::new(GPIO_NUM) {
        led.set_rgb(255, 0, 0); // red

        // async wait
        //led.set_rgb(0, 0, 255, 0, 0); // blue
    }
}
