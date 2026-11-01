use embassy_time::Duration;

use super::{
    Hsv,
    Rgb,
};

/// Implement this for letting your state be shown on the SLed.
pub trait Animator {
    // 'CYCLE' and 'REPEAT' are intentionally left without defaults - avoids surprises.
    const CYCLE: Duration;  // e.g. Duration::from_millis(5_000);
    const REPEAT: Repeat;

    fn sample(&mut self, f: f32) -> Rgb;
}

pub enum Repeat {
    /// Just run the animation one time.
    No,
    /// Repeat forever, 0..1, 0..1, ...
    Yes,
    /// Repeat forever, 0..1, 1..0, ...
    BackAndForth,
}

/// Providing a pulsing (sine wave) transformation between colors 'A' (at 0.0) and 'B' (at 1.0).
///
// Note: Use of const generics like that *did not quite work out* (nightly, 31-Oct-26; Rust 1.101).
//      The problem is that the compiler cannot guarantee only one 'PulseAnimator<A,B>' would be
//      implemented, for the concrete type.
//          <<
//          error[E0207]: the const parameter `A` is not constrained by the impl trait, self type, or predicates
//          --> src/animator.rs:38:9
//          |
//          38 | impl<T, const A: Rgb, const B: Rgb> Animator for T
//          |         ^^^^^^^^^^^^ unconstrained const parameter
//          |
//          = note: expressions using a const parameter must map each value to a distinct output value
//          = note: proving the result of expressions other than the parameter are unique is not supported
//          <<
/***
pub trait PulseAnimator<const A: Rgb, const B: Rgb> {
    const CYCLE: Duration;
    const REPEAT: Repeat;
}

impl<T, const AA: Rgb, const BB: Rgb> Animator for T
where
    T: PulseAnimator<AA,BB>
{
    const CYCLE: Duration = Self::CYCLE;
    const REPEAT: Repeat = Self::REPEAT;

    fn sample(&mut self, f: f32) -> Rgb {
        let _ = f;
        todo!()
    }
}
***/

pub trait PulseAnimator {
    const CYCLE: Duration;
    const REPEAT: Repeat;
}

impl<T, const AA: Rgb, const BB: Rgb> Animator for T
where
    T: PulseAnimator
{
    const CYCLE: Duration = Self::CYCLE;
    const REPEAT: Repeat = Self::REPEAT;

    fn sample(&mut self, f: f32) -> Rgb {
        const A: Rgb = Rgb::BLUE;
        const B: Rgb = Rgb::NONE;

        assert!(f >= 0.0 && f <= 1.0);


        let _ = f;
        todo!()
    }
}

/// A "ping", starting with certain colour and fading out. One time.
// tbd. both linear and exponential implementation (use const generics!)
pub trait PingAnimator {
    const CYCLE: Duration;

    const COLOR: Rgb;
    //const MODE: PingAnimatorMode;     // tbd. Linear | Exponential (default to Exponential)
}

impl<T> Animator for T  // tbd. Would "impl Animator for PingAnimator" do the same???
where
    T: PingAnimator
{
    const CYCLE: Duration = Self::CYCLE;
    const REPEAT: Repeat = Repeat::No;

    fn sample(&mut self, f: f32) -> Rgb {
        let rgb = Self::COLOR;
        const HSV: Hsv = Self::COLOR.into_hsv();

        assert!(f >= 0.0 && f <= 1.0);

        let exp_intensity = (1.0 - f).powf(3.0);

        todo!()
    }
}
