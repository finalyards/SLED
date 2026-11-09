use embassy_time::Duration;

use crate::{Animation, Hsv, Rgb};

/// A "ping", starting with certain 'COLOR' and fading out, exponentially.
///
/// Useful for indicating events in the application code (restart).
///
/// @note Cannot use 'embassy_time::Duration' as the const parameter, since it does not derive
///         'ConstParamTy'.
///
pub struct PingAnimation {
    color: Hsv,
    fade_ms: f32
}

impl PingAnimation {
    pub const fn new(color: Rgb, ms: u32) -> Self {
        let color = color.to_hsv();
        let fade_ms = ms as f32;

        Self{
            color,
            fade_ms
        }
    }

    fn sample_inner(&self, dur: Duration) -> Rgb {
        let f = dur.as_millis() as f32 / self.fade_ms;
        log::debug!("PingAnimation here: dur={}, f={}", dur.as_millis(), f);

        if f >= 1.0 {
            return Rgb::NONE;
        }

        // Note: The power could come from a mode selection, 1.0 being linear.
        let exp_intensity = (1.0 - f).powf(3.0);

        // tbd. TRY OUT: Do we want to:
        //      - animate intensity (value) from its max, _ignoring_ the intensity of the COLOR param
        //          or:
        //      - animate intensity (value) starting from that of the COLOR

        let rgb = {
            let Hsv { h_deg: h, s, v: _ } = self.color;

            #[cfg(true)]
            let v = exp_intensity;  // 1.0 .. 0.0
            #[cfg(false)]
            let v = v * exp_intensity;  // 1.0 .. 0.0

            Hsv { h_deg: h, s, v }.to_rgb()
        };

        //r log::debug!("PingAnimation providing RGB: {:?}", rgb);    // DEBUG
        rgb
    }
}

impl Animation for PingAnimation {
    fn sample(&self, dur: Duration) -> Rgb {
        self.sample_inner(dur)
    }
}
