use alloc::vec::Vec;
use alloc::vec;

use core::marker::ConstParamTy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ConstParamTy)]    // 'PartialEq, Eq' needed for use within const generic
pub struct Rgb{
    pub r:u8,
    pub g:u8,
    pub b:u8
}

impl Rgb {
    pub const NONE: Self = Self { r: 0, g: 0, b: 0 };
}

#[cfg(feature = "color_names")]
impl Rgb {
    // There are quite many of them. https://www.w3schools.com/colors/colors_names.asp
    // This is mainly intended for examples/testing.
    //
    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h_deg: f32,     // 0..<360.0 [degrees]  // #rework: consider keeping this in rad (avoids conversions)
    pub s: f32,     // 0..1.0
    pub v: f32,     // 0..1.0
}

const fn const_max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
const fn const_min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }

impl Rgb {
    /// Convert RGB -> HSV; const
    ///
    /// Use this directly if you prefer 'const fn'. Otherwise, '.into()'.
    pub const fn into_hsv(self) -> Hsv {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let max = const_max(r, const_max(g, b));
        let min = const_min(r, const_min(g, b));
        let delta = max - min;

        // Value
        let v = max;

        // Saturation
        let s = if max == 0.0 { 0.0 } else { delta / max };

        // Hue
        let h_deg = if delta == 0.0 {
            0.0
        } else if max == r {
            let mut h = 60.0 * ((g - b) / delta);
            if h < 0.0 { h += 360.0; }
            h
        } else if max == g {
            60.0 * (((b - r) / delta) + 2.0)
        } else {
            60.0 * (((r - g) / delta) + 4.0)
        };

        Hsv { h_deg, s, v }
    }
}

impl Hsv {
    /// Color with 'v'==0
    const NONE: Self = Self { h_deg: 0.0, s: 0.0, v: 0.0 };

    /// Convert HSV -> RGB (not const; input would anyways be dynamic)
    ///
    fn into_rgb(self) -> Rgb {
        let Hsv { h_deg: h, s, v } = self;

        assert!(h >= 0.0 && h < 360.0);
        assert!(s >= 0.0 && s <= 1.0);
        assert!(v >= 0.0 && v <= 1.0);

        let c = v * s;
        let h_div = h / 60.0;

        let x = c * (1.0 - ((h_div % 2.0) - 1.0).abs());
        let m = v - c;

        let (r_prime, g_prime, b_prime) = if h_div < 1.0 {
            (c, x, 0.0)
        } else if h_div < 2.0 {
            (x, c, 0.0)
        } else if h_div < 3.0 {
            (0.0, c, x)
        } else if h_div < 4.0 {
            (0.0, x, c)
        } else if h_div < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Rgb {
            r: ((r_prime + m) * 255.0).round() as u8,
            g: ((g_prime + m) * 255.0).round() as u8,
            b: ((b_prime + m) * 255.0).round() as u8,
        }
    }

    pub(crate) fn scale_brightness(mut self, factor: f32) -> Self {
        assert!(factor > 0.0 && factor <= 1.0);

        let scaled_v = self.v * factor;

        // Simple gamma correction (v^2) helps tune the brightness to human eye.
        self.v = scaled_v * scaled_v;
        self
    }

    pub(crate) fn merge(x: impl IntoIterator<Item = Hsv>) -> Self {
        merge_with_it(x.into_iter())
    }

    // #hack Until the tests would be done differently.
    //
    pub fn _merge_for_tests(x: impl IntoIterator<Item = Hsv>) -> Self {
        merge_with_it(x.into_iter())
    }
}

/// Merge multiple HSV colors together. This is done as-if the lit colors merged naturally,
/// and tuned down so that the resulting brightness stays within expressible domain.
///
fn merge_with_it(iter: impl Iterator<Item = Hsv>) -> Hsv {
    // Ignore anything with 'v' == 0.0.
    let it = {
        let mut pit = iter
            .filter(|&c| c.v > 0.0)
            .peekable();

        if pit.peek().is_none() {
            return Hsv::NONE;
        } else {
            pit
        }
    };

    // H: sin/cos average of the hues, weighed with 's * v' (i.e. pure colors get 1.0 weight;
    //    e.g. white gets 0.0 weight and its entry is not involved in hue decision-making.
    // S: avg of s's, weighed with 'v'
    // V: max of the involved (let's try this way...)
    //
    let mut sin_sum = 0.0_f32;
    let mut cos_sum = 0.0_f32;
    let mut hue_weight_sum = 0.0_f32;

    let mut s_weighted_sum = 0.0_f32;
    let mut v_sum = 0.0;
    let mut v_max = 0.0;

    for Hsv { h_deg, s, v } in it {
        assert!(v > 0.0);   // already excluded

        // Hue weight: colorful (s > 0) and bright (v > 0) influence most
        let hue_weight = v * s;
        if hue_weight > 0.0 {
            let rad = h_deg.to_radians();
            sin_sum += rad.sin() * hue_weight;
            cos_sum += rad.cos() * hue_weight;
            hue_weight_sum += hue_weight;
        }

        // Saturation weight: brighter influences saturation more
        s_weighted_sum += s * v;

        v_sum += v;
        v_max = f32::max(v_max, v);
    }

    assert!(v_sum > 0.0);   // checked at the beginning that someone has 'v'

    // Calculate the weighted 's' (if all white; s=0; it's 0)
    let h_avg = if hue_weight_sum > 0.0 {
        let deg = sin_sum.atan2(cos_sum).to_degrees();
        if deg < 0.0 { deg + 360.0 } else { deg }
    } else {
        0.0
    };

    // Weighted saturation
    let s_avg = s_weighted_sum / v_sum;

    Hsv {
        h_deg: h_avg,
        s: s_avg,
        v: v_max,
    }
}

#[cfg(false)]   //r: earlier
fn merge_with_it(iter: impl Iterator<Item = Hsv>) -> Hsv {
    // Ignore anything with 'v' == 0.0: they (not only black) should not affect the mix.
    //
    let it = {
        let mut pit = iter
            .filter(|&c| c.v > 0.0)
            .peekable();

        if pit.peek().is_none() {
            return Hsv::NONE;
        } else {
            pit
        }
    };

    // H: sin/cos average of the hues, weighed with 's * v' (i.e. pure colors get 1.0 weight;
    //    e.g. white gets 0.0 weight and its entry is not involved in hue decision-making.
    // S: avg of s's, weighed with 'v'
    // V: ...tbd...
    //
    let mut hs_rad: Vec<f32> = vec!();
    let mut s_sum = 0.0;
    //let mut v_sum = 0.0;
    let mut v_max = 0.0;

    for Hsv{ h_deg,s,v} in it {
        let rad = h_deg.to_radians();
        hs_rad.push(rad);

        s_sum += s;
        //v_sum += v;
        v_max = f32::max(v_max, v);
    }

    let len = hs_rad.len();

    let h_avg = {
        let mut sin_sum = 0.0_f32;
        let mut cos_sum = 0.0;

        for rad in hs_rad {
            sin_sum += rad.sin();
            cos_sum += rad.cos();
        }

        let rad_avg = sin_sum.atan2(cos_sum);

        let deg = rad_avg.to_degrees();
        if deg < 0.0 { deg + 360.0 } else { deg }
    };

    let s_avg = s_sum / len as f32;

    Hsv {
        h_deg: h_avg,
        s: s_avg,
        v: v_max,
    }
}


impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        rgb.into_hsv()
    }
}

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        hsv.into_rgb()
    }
}

/// Convert HSV -> RGB
///
/// @note We don't benefit from this being 'const fn'. If there's a better non-const way, can use it.
///
#[cfg(false)]
pub const fn to_rgb(self) -> Rgb {
    // Clamp to limits in 'const' context
    let h = if self.h < 0.0 { 0.0 } else if self.h >= 360.0 { 359.9 } else { self.h };
    let s = if self.s < 0.0 { 0.0 } else if self.s > 1.0 { 1.0 } else { self.s };
    let v = if self.v < 0.0 { 0.0 } else if self.v > 1.0 { 1.0 } else { self.v };

    let c = v * s;
    // const compatible modulo operation for floating point: (h / 60.0) % 2.0
    let h_div = h / 60.0;
    let rem = h_div - (2.0 * ((h_div / 2.0) as i32 as f32));
    let x = c * (1.0 - if rem - 1.0 < 0.0 { -(rem - 1.0) } else { rem - 1.0 });
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h_div < 1.0 {
        (c, x, 0.0)
    } else if h_div < 2.0 {
        (x, c, 0.0)
    } else if h_div < 3.0 {
        (0.0, c, x)
    } else if h_div < 4.0 {
        (0.0, x, c)
    } else if h_div < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Rgb {
        r: ((r_prime + m) * 255.0 + 0.5) as u8,
        g: ((g_prime + m) * 255.0 + 0.5) as u8,
        b: ((b_prime + m) * 255.0 + 0.5) as u8,
    }
}
