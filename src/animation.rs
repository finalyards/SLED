
use core::{
    cmp::Ordering,
    ops::Deref,
};

use embassy_time::Duration;

use super::{
    Hsv,
    Rgb,
    SLed,
};

pub use ping::PingAnimation;

/// Implement this for letting your state be shown on the SLed.
///
/// @note Must be dyn compatible (used as 'dyn Animation' by SLed).
///
pub trait Animation : Sync {

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
}

impl dyn Animation {
    /// @task Application
    ///
    /// Restarts an animation from the beginning (next frame will be 0.0).
    ///
    /// @note The animated change is abrupt. (Eventually, we might have smoothing between change of
    ///     *states* - here there is only one state). tbd. reword after state-changing is implemented.
    ///
    pub fn restart(self: &'static dyn Animation) {
        let a_key = AnimationKey(self);
        SLed::reset(a_key)
    }
}

// Specific animations
mod ping {
    use super::*;

    /// A "ping", starting with certain 'COLOR' and fading out. One time, but can be restarted.
    ///
    /// @note Represents events.
    pub trait PingAnimation: Sync {
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

/// Wrapper on '&'static dyn Animation' to use them as keys in 'BTreeMap'.
///
#[derive(Clone, Copy)]
pub struct AnimationKey(pub &'static dyn Animation);

impl PartialEq for AnimationKey {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq(
            self.0 as *const dyn Animation as *const (),
            other.0 as *const dyn Animation as *const ()
        )
    }
}

impl Eq for AnimationKey {}

impl PartialOrd for AnimationKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnimationKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_ptr = self.0 as *const dyn Animation as *const ();
        let other_ptr = other.0 as *const dyn Animation as *const ();
        self_ptr.cmp(&other_ptr)
    }
}

// Allow using the wrapper just like the reference.
impl Deref for AnimationKey {
    type Target = dyn Animation;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}
