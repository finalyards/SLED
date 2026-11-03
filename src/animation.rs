
use embassy_time::Duration;

use super::{
    Hsv,
    Rgb,
    SLed,
};

/// Implement this for letting your state be shown on the SLed.
///
/// @note Must be dyn compatible (used as 'dyn Animation' by SLed).
///
pub trait Animation {

    /// @task SLed
    /// Called by the 'run_loop', on regular intervals.
    ///
    /// @param dur: Duration since the beginning of the animation.
    ///
    /// @note Returning black (r= g= b= 0) counts as no output - it will not affect other colors
    ///     in the mixing.
    ///
    // Rust note: `&self` is required so this method can be dispatched dynamically via `dyn Animation`.
    fn sample(&self, dur: Duration) -> Rgb;

    /// @task Application
    ///
    /// Restarts an animation from the beginning (next frame will be 0.0).
    ///
    /// This can be used for one-off animations, to run them again. Or repeating animations, where
    /// it does the same: starts from 0.0.
    ///
    /// @note The animated change is abrupt. (Eventually, we might have smoothing between change of
    ///     *states* - here there is only one state). tbd. reword after state-changing is implemented.
    ///
    fn restart(&self) {
        SLed::restart(&self)
    }
}

// Specific animations
mod ping {
    use super::*;

    /// A "ping", starting with certain 'COLOR' and fading out. One time, but can be restarted.
    ///
    /// @note Represents events.
    pub trait PingAnimation {
        const CYCLE: Duration;
        //const MODE: Linear | Exponential  // tbd. experiment; we can have a default mode
        const COLOR: Rgb;
    }

    impl<T> Animation for T
    where
        T: PingAnimation
    {
        fn sample(&self, dur: Duration) -> Rgb {
            let f = dur.as_millis() as f32 / Self::CYCLE.as_millis() as f32;

            if f >= 1.0 {
                return Rgb::NONE;   // one shot
            }

            let color_hsv = Self::COLOR.to_hsv();
            //const COLOR_HSV: Hsv = Self::COLOR.to_hsv();
                //
                // Rust constness note: could not use a 'const' here, as a target. The value should
                //          be evaluated as const, however, since '.to_hsv()' is 'const fn'.

            // Note: The power could come from a mode selection, 1.0 being linear.
            let exp_intensity = (1.0 - f).powf(3.0);

            // tbd. TRY OUT: Do we want to:
            //      - animate intensity (value) from its max, _ignoring_ the intensity of the COLOR param
            //          or:
            //      - animate intensity (value) starting from that of the COLOR

            let rgb = {
                let Hsv { h, s, v: _ } = color_hsv;

                #[cfg(true)]
                let v = exp_intensity;  // 1.0 .. 0.0
                #[cfg(false)]
                let v = v * exp_intensity;  // 1.0 .. 0.0

                Hsv { h, s, v }.to_rgb()
            };

            rgb
        }
    }
}
