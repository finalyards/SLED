
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{
        Channel,
    }
};
use embassy_time::{Duration, Timer};
use esp_idf_hal::gpio::{PinDriver, Input};

#[derive(Debug, Copy, Clone)]
pub enum ButtonEvent {
    Pressed,
    Depressed,
}

// Using 'CriticalSectionRawMutex' is a must. It grants 'Sync'.
static BUTTON_CHANNEL: Channel<CriticalSectionRawMutex, ButtonEvent, 4> = Channel::new();

/**
* Listens to the BOOT button, reflecting its state to 'BUTTON_CHANNEL'
*/
#[embassy_executor::task]
pub async fn button_task(mut btn: PinDriver<'static, Input>) {
    const DEBOUNCE_DUR: Duration = Duration::from_millis(50);

    async fn debounce_wait() {
        Timer::after(DEBOUNCE_DUR).await;
    }
    let tx = BUTTON_CHANNEL.sender();

    loop {
        loop {
            if btn.wait_for_low().await.is_ok() {
                debounce_wait() .await;
                if btn.is_low() { break; }
            }
        }
        tx.send(ButtonEvent::Pressed).await;

        loop {
            if btn.wait_for_high().await.is_ok() {
                debounce_wait() .await;
                if btn.is_high() { break; }
            }
        }
        tx.send(ButtonEvent::Depressed).await;
    }
}

/**
*/
pub async fn receive() -> ButtonEvent {
    BUTTON_CHANNEL.receiver().receive() .await
}
