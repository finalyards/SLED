use core::{
    fmt,
    num::NonZeroI32,
    ptr,
};

#[path = "../bindings.rs"]
mod bindings;

use bindings::{
    esp_err_t,
    led_strip_config_t,
    led_strip_handle_t,
    led_strip_new_rmt_device,
    led_strip_rmt_config_t,
    led_strip_set_pixel,
    led_strip_refresh,
    //led_strip_clear,
    led_model_t,
    soc_periph_rmt_clk_src_t,
};
#[cfg(feature = "_color_mode_grb")]
use bindings::{
    wrap_LED_STRIP_COLOR_COMPONENT_FMT_GRB,
};

use super::Rgb;

/// Handles connection with the ESP-IDF 'espressif/led_strip' component.
pub(crate) struct RawSLed {
    handle: led_strip_handle_t,
}

impl RawSLed {
    #[cfg(feature = "_gpio_8")]
    const GPIO_NUM: u8 = 8;

    pub fn new() -> Result<Self, EspErr> {
        let mut handle: led_strip_handle_t = ptr::null_mut();

        let led_cfg = led_strip_config_t {
            strip_gpio_num: Self::GPIO_NUM as i32,
            max_leds: 1,    // just 1
            led_model: led_model_t::LED_MODEL_WS2812,
            color_component_format: {
                #[cfg(feature = "_color_mode_grb")]
                unsafe { wrap_LED_STRIP_COLOR_COMPONENT_FMT_GRB() }
                #[cfg(not(feature = "_color_mode_grb"))]
                unimplemented!()
            },
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

    // Note: Likely the underlying 'led_strip' functions never err, but we cannot know for sure.
    //  Their docs state "because other error occurred".
    //
    //  tbd. Study the 'led_strip' sources; decide, whether we should panic already here (and offer
    //      more straightforward API).

    pub fn set_rgb(&self, rgb: Rgb) -> Result<(), EspErr> {
        let Rgb{ r, g, b } = rgb;

        EspErr::wrap( unsafe {
            // component makers have decided to keep the API wide. In practise, only the 8 LSBs matter.
            led_strip_set_pixel(self.handle, 0, r as u32, g as u32, b as u32)
        })?;

        EspErr::wrap( unsafe {
            led_strip_refresh(self.handle)
        })
    }

    #[cfg(false)]   // disabled
    pub fn turn_off(&self) -> Result<(), EspErr> {
        EspErr::wrap( unsafe {
            led_strip_clear(self.handle)
        })
    }
}

/// Wrapping of 'esp_err_t'
#[derive(Debug)]
pub(crate) struct EspErr(pub NonZeroI32 /*esp_err_t*/);

impl EspErr {
    pub(crate) fn wrap(err: esp_err_t) -> Result<(),Self> {
        match NonZeroI32::new(err) {
            None => Ok(()),
            Some(x) => Err(Self(x)),
        }
    }
}

impl fmt::Display for EspErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let ee = esp_idf_sys::EspError::from(self.0.get()).unwrap();
        write!(f, "{}", ee)
    }
}

impl core::error::Error for EspErr {}
