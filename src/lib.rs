
use esp_idf_svc::sys::{
    led_strip_config_t,
    led_strip_handle_t,
    led_strip_new_rmt_device,
    led_strip_rmt_config_t,
    led_strip_set_pixel,
    led_strip_refresh,
    led_strip_clear
};
use std::ptr;

pub struct BuiltInLed {
    handle: led_strip_handle_t,
}

impl BuiltInLed {
    pub fn new(gpio_num: u8) -> Result<Self, EspErr> {
        let mut handle: led_strip_handle_t = ptr::null_mut();

        let led_cfg = led_strip_config_t {
            strip_gpio_num: gpio_num as i32,
            max_leds: 1,    // just the 1
            led_model: led_model_t_LED_MODEL_WS2812,    // 0
            color_component_format: wrap_LED_STRIP_COLOR_COMPONENT_FMT_GRB(),
            ..Default::default()
                // no invert
        };

        let rmt_cfg = led_strip_rmt_config_t {
            clk_src: rmt_clock_source_t:: soc_periph_rmt_clk_src_t_RMT_CLK_SRC_DEFAULT,
            //resolution_hz: 0 or 10_000_000, // 10 MHz (default)
            //mem_block_symbols: 0,     // (default); ESP-IDF driver decides
            //flags: ...                // (default) no DMA needed
            ..Default::default()
        };

        let err = unsafe {
            led_strip_new_rmt_device(&led_cfg, &rmt_cfg, &mut handle)
        };
        EspErr::from(err)?;

        assert!(!handle.is_null(), "LED handle is NULL");

        Ok(Self { handle })
    }

    pub fn set_rgb(&self, r: u32, g: u32, b: u32) {
        unsafe {
            // Indeksi 0 = ensimmäinen (ja ainoa) ledi
            led_strip_set_pixel(self.handle, 0, r, g, b);
            // Työnnetään data ledille RMT-väylää pitkin
            led_strip_refresh(self.handle);
        }
    }

    pub fn turn_off(&self) {
        unsafe {
            led_strip_clear(self.handle);
        }
    }
}

pub struct EspErr(pub u8);

impl EspErr {
    fn from(err: u8) -> Option<Self> {
        if err==0 { None }
        else { Some(Self(err)) }
    }
}
