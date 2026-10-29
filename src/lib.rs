#![no_std]

use core::ptr;

#[path = "../bindings.rs"]
mod bindings;

use bindings::{
    led_strip_config_t,
    led_strip_handle_t,
    led_strip_new_rmt_device,
    led_strip_rmt_config_t,
    led_strip_set_pixel,
    led_strip_refresh,
    led_strip_clear,
    led_model_t,
    wrap_LED_STRIP_COLOR_COMPONENT_FMT_GRB,
    soc_periph_rmt_clk_src_t,
};

mod error;
pub use error::EspErr;

// It's needed at the build, on the 'dependencies' side. This removes a warning.
use esp_idf_sys as _;

pub struct SLed {
    handle: led_strip_handle_t,
}

impl SLed {
    pub fn new(gpio_num: u8) -> Result<Self, EspErr> {
        let mut handle: led_strip_handle_t = ptr::null_mut();

        let led_cfg = led_strip_config_t {
            strip_gpio_num: gpio_num as i32,
            max_leds: 1,    // just the 1
            led_model: led_model_t::LED_MODEL_WS2812,    // 0
            color_component_format: unsafe { wrap_LED_STRIP_COLOR_COMPONENT_FMT_GRB() },
            ..Default::default()
                // no invert
        };

        let rmt_cfg = led_strip_rmt_config_t {
            clk_src: soc_periph_rmt_clk_src_t::RMT_CLK_SRC_DEFAULT,   // PLL_F80M
            //resolution_hz: 0 or 10_000_000, // 10 MHz (default)
            //mem_block_symbols: 0,     // (default); ESP-IDF driver decides
            //flags: ...                // (default) no DMA needed
            ..Default::default()
        };

        EspErr::wrap( unsafe {
            led_strip_new_rmt_device(&led_cfg, &rmt_cfg, &mut handle)
        })?;
        assert!(!handle.is_null(), "LED handle is NULL");

        Ok(Self { handle })
    }

    pub fn set_rgb(&self, r: u8, g: u8, b: u8) {
        unsafe {
            // component makers have decided to keep the API wide. In practise, only the 8 LSBs matter.
            led_strip_set_pixel(self.handle, 0, r as u32, g as u32, b as u32);
            led_strip_refresh(self.handle);
        }
    }

    pub fn turn_off(&self) {
        unsafe {
            led_strip_clear(self.handle);
        }
    }
}
