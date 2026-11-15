use embassy_executor::Spawner;

use esp_idf_svc::{
    sys::link_patches,
};

mod hsv_test;

/// Example that tests color logic
///
/// Such tests *could* be host-driven (there's nothing embedded in 'src/color.rs', but we'd need to
/// detach the said file from the lib itself. For the moment, the author prefers the simplicity of
/// not using a workspace project.
///
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    // Log level defined by 'RUST_LOG' env.var.
    esp_idf_svc::log::init_from_env();

    log::info!("HSV tests");
    hsv_test::run_all();

    // tbd. how to graciously exit to host?  Can we; not worth much effort.
    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}
