use alloc::vec::Vec;
use alloc::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]    // 'PartialEq, Eq' needed for use within const generic
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
    pub const fn to_hsv(self) -> Hsv {
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
    /// Convert HSV -> RGB (not const; input would anyways be dynamic)
    ///
    pub fn to_rgb(self) -> Rgb {
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

    pub(crate) fn merge_from(x: impl IntoIterator<Item = Hsv>) -> Self {
        merge(x.into_iter())
    }
}

fn merge(iter: impl Iterator<Item = Hsv>) -> Hsv {
    // Filter out anything with 'v' == 0.0: they (not only black) should not affect the mix.
    //
    let mut colors_iter = iter
        .filter(|&c| c.v > 0.0)
        .peekable();

    if colors_iter.peek().is_none() {
        return Hsv { h_deg: 0.0, s: 0.0, v: 0.0 };
    }

    let mut hs_rad: Vec<f32> = vec!();
    let mut s_sum = 0.0;
    //let mut v_sum = 0.0;
    let mut v_max = 0.0;

    for Hsv{ h_deg: h,s,v} in colors_iter {
        let rad = h.to_radians();
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
        let tmp = rad_avg.to_degrees();
        if tmp < 0.0 { tmp + 360.0 } else { tmp }
    };

    let s_avg = s_sum / len as f32;

    Hsv {
        h_deg: h_avg,
        s: s_avg,
        v: v_max,
    }
}

//#[test]
impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        rgb.to_hsv()
    }
}

//#[test]
impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        hsv.to_rgb()
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.001;

    // Apufunktio HSV-arvojen vertailuun liukulukkopyöristykset huomioiden
    fn assert_hsv_near(actual: Hsv, expected_h_deg: f32, expected_s: f32, expected_v: f32) {
        // Jos kyseessä on musta tai täysin harmaa, sävyllä (H) ei ole merkitystä,
        // joten tarkistetaan se vain jos saturaatio tai kirkkaus on merkittävä.
        if expected_v > EPSILON && expected_s > EPSILON {
            assert!((actual.h_deg - expected_h_deg).abs() < 1.0 /*deg*/, "Hue is off: {} vs. {}", actual.h_deg, expected_h_deg);
        }
        assert!((actual.s - expected_s).abs() < EPSILON, "Saturation is off: {} vs. {}", actual.s, expected_s);
        assert!((actual.v - expected_v).abs() < EPSILON, "Value is off: {} vs. {}", actual.v, expected_v);
    }

    #[test]
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
    }

    #[test]
    fn test_rgb_to_hsv_black_and_white() {
        // Black (LED off): RGB(0, 0, 0) -> V 0.0 (S,V ignored)
        let black_hsv = Hsv::from(Rgb { r: 0, g: 0, b: 0 });
        assert_hsv_near(black_hsv, 0.0, 0.5, 0.5);

        // White: RGB(255, 255, 255) -> Saturation 0.0, Value 1.0 (H ignored)
        let white_hsv = Hsv::from(Rgb { r: 255, g: 255, b: 255 });
        assert_hsv_near(white_hsv, 0.0, 0.0, 1.0);
    }

    #[test]
    fn test_hsv_to_rgb_roundtrip() {
        // Testataan, että muunnos HSV -> RGB palauttaa alkuperäiset arvot
        let targets = vec![
            Rgb { r: 255, g: 0, b: 0 },   // Red
            Rgb { r: 0, g: 255, b: 0 },   // Green
            Rgb { r: 0, g: 0, b: 255 },   // Blue
            Rgb { r: 128, g: 128, b: 128 }, // Gray
            Rgb { r: 255, g: 128, b: 0 },   // Orange
        ];

        for original_rgb in targets {
            let hsv = Hsv::from(original_rgb);
            let roundtrip_rgb = Rgb::from(hsv);
            assert_eq!(original_rgb, roundtrip_rgb, "Roundtrip failed: {:?} -> {:?} -> {:?}", original_rgb, hsv, roundtrip_rgb);
        }
    }
}
