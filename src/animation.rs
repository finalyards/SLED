
use core::{
    cmp::Ordering,
    ops::Deref,
};

use embassy_time::Duration;

use super::{
    Rgb,
    reset_anim,
};

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
        reset_anim(a_key)
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
