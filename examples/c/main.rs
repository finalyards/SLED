use embassy_executor::{
    Spawner
};

use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

use esp_idf_svc::{
    sys::link_patches,
};

use embassy_time::{
    Duration,
    Instant,
};

mod button_task;
use button_task::button_task;
use sled::SLed;

mod states;
use states::ButtonPing;

//static BLUE_WAVE_ANIMATION: BlueWave = BlueWave::new();
static PING_ANIMATION: ButtonPing = ButtonPing::new();

/// Example with:
///     - multiple states
///     - interactive state change
///
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Lights up, world!");

    let peripherals = Peripherals::take().unwrap();
    {
        // Set GPIO9 (BOOT btn) as input (pressed is Low)
        let btn_pin = PinDriver::input(peripherals.pins.gpio9, Pull::Up).unwrap();

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
    let states = &[&PING_ANIMATION];

    let led = SLed::new(states)
        .unwrap_or_else(|e| panic!("Failed to initialize: {:?}", e));

    led.run_loop() .await;
}
