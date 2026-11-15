use sled::{Hsv, Rgb};

const EPSILON: f32 = 0.001;

pub fn run_all() {
    test_rgb_to_hsv_primary_colors();
    test_rgb_to_hsv_black_and_white();
    test_hsv_to_rgb_roundtrip();
    test_hsv_merge();
}

/// Test merging of colors. Also documents the way we would like merging to happen.
///
fn test_hsv_merge() {
    const BLUE: Hsv = Rgb::BLUE.into_hsv();     // { h_deg: 240.0, s: 1.0, v: 1.0 }
    const WHITE: Hsv = Rgb::WHITE.into_hsv();    // { s: 0.0, v: 1.0, .. }

    #[cfg(false)]
    log::/*debug*/ info!("BLUE: {:?}, WHITE: {:?}", BLUE, WHITE);

    // The test cases are simple, but covering them all should lead to a mixing function that
    // behaves like we want.

    // BLUE + WHITE -> paler blue
    {
        let Hsv { h_deg, s, v } = Hsv::_merge_for_tests([BLUE, WHITE]);

        assert_eq!(h_deg, BLUE.h_deg);
        assert_eq!(s, 0.5);     // tbd. with tolerance
        assert_eq!(v, (BLUE.v + WHITE.v) /2.0);
    }

    // tbd.

    log::info!("test_hsv_merge: pass");
}

fn test_rgb_to_hsv_primary_colors() {
    // R: RGB(255, 0, 0) -> HSV(0.0, 1.0, 1.0)
    let red_hsv = Hsv::from(Rgb { r: 255, g: 0, b: 0 });
    assert_hsv_near(red_hsv, 0.0, 1.0, 1.0);

    // G: RGB(0, 255, 0) -> HSV(120.0, 1.0, 1.0)
    let green_hsv = Hsv::from(Rgb { r: 0, g: 255, b: 0 });
    assert_hsv_near(green_hsv, 120.0, 1.0, 1.0);

    // B: RGB(0, 0, 255) -> HSV(240.0, 1.0, 1.0)
    let blue_hsv = Hsv::from(Rgb { r: 0, g: 0, b: 255 });
    assert_hsv_near(blue_hsv, 240.0, 1.0, 1.0);

    log::info!("test_rgb_to_hsv_primary_colors: pass");
}

fn test_rgb_to_hsv_black_and_white() {
    // Black (LED off): RGB(0, 0, 0) -> V 0.0 (S,V ignored)
    let black_hsv = Hsv::from(Rgb { r: 0, g: 0, b: 0 });
    assert_h_near(black_hsv, 0.0);

    // White: RGB(255, 255, 255) -> Saturation 0.0, Value 1.0 (H ignored)
    let white_hsv = Hsv::from(Rgb { r: 255, g: 255, b: 255 });
    assert_sv_near(white_hsv, 0.0, 1.0);

    log::info!("test_rgb_to_hsv_black_and_white: pass");
}

fn test_hsv_to_rgb_roundtrip() {
    let targets = vec![
        Rgb { r: 255, g: 0, b: 0 },     // Red
        Rgb { r: 0, g: 255, b: 0 },     // Green
        Rgb { r: 0, g: 0, b: 255 },     // Blue
        Rgb { r: 128, g: 128, b: 128 }, // Gray
        Rgb { r: 255, g: 128, b: 0 },   // Orange
    ];

    for original_rgb in targets {
        let hsv = Hsv::from(original_rgb);
        let roundtrip_rgb = Rgb::from(hsv);
        assert_eq!(original_rgb, roundtrip_rgb, "Roundtrip failed: {:?} -> {:?} -> {:?}", original_rgb, hsv, roundtrip_rgb);
    }

    log::info!("test_hsv_to_rgb_roundtrip: pass");
}

//--- Helpers

/// Compare HSV values (all deemed important), considering floating point inaccuracies.
///
/// @note Could be improved such that all HSV "blind spots" (where a certain value deems another ignored) are taken into
///         account. Currently, one is. Others fall to the caller.
///
fn assert_hsv_near(actual: Hsv, expected_h_deg: f32, expected_s: f32, expected_v: f32) {
    // If black or fully gray, hue (H) has no impact.
    if expected_v > EPSILON && expected_s > EPSILON {
        assert_h_near(actual, expected_h_deg);
    }
    assert_sv_near(actual, expected_s, expected_v);
}
fn assert_h_near(actual: Hsv, expected_h_deg: f32) {
    assert!((actual.h_deg - expected_h_deg).abs() < 1.0 /*deg*/, "Hue is off: {} vs. {}", actual.h_deg, expected_h_deg);
}
fn assert_sv_near(actual: Hsv, expected_s: f32, expected_v: f32) {
    assert!((actual.s - expected_s).abs() < EPSILON, "Saturation is off: {} vs. {}", actual.s, expected_s);
    assert!((actual.v - expected_v).abs() < EPSILON, "Value is off: {} vs. {}", actual.v, expected_v);
}
