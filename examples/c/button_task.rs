
use embassy_time::{Duration, Timer};
use esp_idf_hal::gpio::{PinDriver, Input};

use sled::Animation;

/**
* Listens to the BOOT button, reflecting presses in 'state'.
*/
#[embassy_executor::task]
pub async fn button_task(mut btn: PinDriver<'static, Input>, state: &'static dyn Animation) {
    const DEBOUNCE_DUR: Duration = Duration::from_millis(50);

    async fn debounce_wait() {
        Timer::after(DEBOUNCE_DUR).await;
    }

    log::debug!("Starting to track the button..");
    loop {
        loop {
            if btn.wait_for_low().await.is_ok() {
                debounce_wait() .await;
                if btn.is_low() { break; }
            }
        }
        log::debug!("Button PRESSED");
        // Trigger on press
        state.restart();

        loop {
            if btn.wait_for_high().await.is_ok() {
                debounce_wait() .await;
                if btn.is_high() { break; }
            }
        }
        // nothing on raise
    }
}
