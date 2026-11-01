#[allow(unstable_features)]

use core::marker::ConstParamTy;

#[derive(PartialEq, Eq, ConstParamTy, Copy, Clone, Debug)]
pub struct Rgb{
    pub r:u8,
    pub g:u8,
    pub b:u8
}

impl Rgb {
    // There are quite many of them. https://www.w3schools.com/colors/colors_names.asp
    // This is mainly intended for examples/testing.
    //
    // Note: Prefer 'NONE' to 'BLACK' since we're not dimming other colors if one is black.
    pub const NONE: Self = Self { r: 0, g: 0, b: 0 };

    pub const RED: Self = Self { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Self { r: 0, g: 0, b: 255 };

    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    /// Create an rgb color from a hex constant, e.g. '0xff7f50' ("coral").
    #[cfg(false)]
    const fn from_hex(v: u32) -> Self {
        Self{
            r: (v >> 16) as u8,
            g: ((v >> 8) & 0xff) as u8,
            b: (v & 0xff) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h: f32,     // 0..<360.0 [degrees]
    pub s: f32,     // 0..1.0
    pub v: f32,     // 0..1.0
}

const fn const_max(a: f32, b: f32) -> f32 { if a > b { a } else { b } }
const fn const_min(a: f32, b: f32) -> f32 { if a < b { a } else { b } }

impl Rgb {
    /// Convert RGB -> HSV
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
        let h = if delta == 0.0 {
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

        Hsv { h, s, v }
    }
}

impl Hsv {
    /// Convert HSV -> RGB
    ///
    /// @note We don't need to be 'const fn'
    ///
    pub fn to_rgb(self) -> Rgb {
        let Hsv { h, s, v } = self;

        assert!( h >= 0.0 && h < 360.0 );
        assert!( s >= 0.0 && s <= 1.0 );
        assert!( v >= 0.0 && v <= 1.0 );

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
}
