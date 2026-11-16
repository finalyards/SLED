
use embassy_time::{Duration, Timer};
use esp_idf_hal::gpio::{PinDriver, Input};

use sled::Animation;

/**
* Listens to the BOOT button, reflecting presses in 'state'.
*/
#[embassy_executor::task]
pub async fn button_task(mut btn: PinDriver<'static, Input>, state: &'static dyn Animation) {
    const DEBOUNCE_DUR: Duration = Duration::from_millis(50);

    /***R
    /// Log a button error and wait for a moment, so that there would not be a flood of such
    /// messages. (Likely we never have any; what could be the cause? tbd.)
    async fn error_wait(e: esp_idf_sys::EspError) {
        log::error!("Error waiting for the BOOT button: {}", e);
        Timer::after( Duration::from_millis(1000) ).await;
    }*/

    log::info /*debug*/!("♦️Starting to track the button..");
    loop {
        loop {
            // Reading the pin would fail only on the _first_ call, if GPIO interrupt fails to get
            // registered.
            btn.wait_for_falling_edge()
                .await
                .expect("Failure to register GPIO interrupt for BOOT button");

            Timer::after(DEBOUNCE_DUR).await;
            if btn.is_low() { break; }
        }

        log::info /*debug*/!("🍏Button PRESSED");
        // Trigger on press
        state.restart();

        // Above '.wait_for_falling_edge()' will wait over the "high" state - including possible
        //      bounces in the transition to it.
    }
}
