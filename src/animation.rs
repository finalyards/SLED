use embassy_time::Duration;

use super::{
    Hsv,
    Rgb,
    SLed,
};

/// Implement this for letting your state be shown on the SLed.
pub trait Animation {
    /// Cycle length of one animation round (0.0 -> 1.0).
    // Note: Need to use a function (not 'const CYCLE') to remain "dyn compatible". Upper traits
    //      may, however (and do), provide 'const CYCLE: Duration' to the application.
    fn cycle(&self) -> Duration;
    // const CYCLE: Duration;

    //r? const REPEAT: Repeat;

    fn sample(&mut self, f: f32) -> Option<Rgb>;

    /// Restarts the animation from the beginning (next frame will be 0.0).
    ///
    /// This can be used for one-off animations, to run them again. Or repeating animations, where
    /// it does the same: starts from 0.0.
    ///
    /// There is no smoothing (once we implement those for state changes); the state does not
    /// actually change, just the animation *showing* that state is restarted.
    ///
    fn restart(&self) {
        SLed::restart(&self)
    }
}

#[cfg(false)]   //? #later; perhaps another way
pub enum Repeat {
    /// Just run the animation one time.
    No,
    /// Repeat forever, 0..1, 0..1, ...
    Yes,
    /// Repeat forever, 0..1, 1..0, ...
    BackAndForth,
}

/// A "ping", starting with certain 'COLOR' and fading out. One time, but can be restarted.
pub trait PingAnimation {
    const CYCLE: Duration;

    const COLOR: Rgb;
}

impl<T> Animation for T
where
    T: PingAnimation
{
    //const CYCLE: Duration = Self::CYCLE;
    fn cycle(&self) -> Duration { Self::CYCLE }

    fn sample(&mut self, f: f32) -> Option<Rgb> {
        assert!(f >= 0.0 && f <= 1.0);

        let color_hsv: Hsv = Self::COLOR.to_hsv();

        let exp_intensity = (1.0 - f).powf(3.0);

        // tbd. TRY OUT. Do we want to just take hue and saturation from 'COLOR' and ping full...nothing.
        #[cfg(true)]
        let rgb = {
            let Hsv { h, s, v: _ } = color_hsv;

            let v = exp_intensity;  // 1.0 .. 0.0

            Hsv { h, s, v }.to_rgb()
        };
        // ..or fade from the intensity provided.
        #[cfg(false)]
        {
            let Hsv { h, s, mut v } = Self::_COLOR_HSV;

            let v = v * exp_intensity;  // 1.0 .. 0.0

            Hsv { h, s, v }.to_rgb()
        }

        Some(rgb)
    }
}
