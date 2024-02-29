use std::ops::{Add, Mul, Sub};

use num::{FromPrimitive, Num, NumCast};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    pub const fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0);
    pub const RED: Colour = Colour::new(1.0, 0.0, 0.0);

    pub fn scale<T: Num + FromPrimitive + NumCast + Copy>(&self, scale: T) -> (T, T, T) {
        let f = |c: f64, s: f64| -> T {
            if c < 0.0 {
                T::zero()
            } else if c > 1.0 {
                scale
            } else {
                T::from_f64((c * s).ceil()).unwrap_or(T::zero())
            }
        };
        let s = T::to_f64(&scale).unwrap_or(0.0);
        (f(self.r, s), f(self.g, s), f(self.b, s))
    }
}

impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub<Colour> for Colour {
    type Output = Colour;

    fn sub(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, rhs: f64) -> Self::Output {
        Colour::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() -> () {
        let c = Colour::new(1.0, 0.5, 0.5);
        let scale = 12;
        let (r8, g8, b8) = c.scale::<u8>(scale);
        assert_eq!(r8, scale);
        assert_eq!(g8, 6);
        assert_eq!(b8, 6);
        let (r16, g16, b16) = c.scale::<u16>(scale as u16);
        assert_eq!(r16, scale as u16);
        assert_eq!(g16, 6);
        assert_eq!(b16, 6);
        let (r32, g32, b32) = c.scale::<u32>(scale as u32);
        assert_eq!(r32, scale as u32);
        assert_eq!(g32, 6);
        assert_eq!(b32, 6);
        let (r64, g64, b64) = c.scale::<u64>(scale as u64);
        assert_eq!(r64, scale as u64);
        assert_eq!(g64, 6);
        assert_eq!(b64, 6);
    }

    #[test]
    fn scale_gt1_is_scale() -> () {
        let c = Colour::new(1.0, 2.0, 20.0);
        let scale = 12;
        let (r, g, b) = c.scale(scale);
        assert_eq!(r, scale);
        assert_eq!(g, scale);
        assert_eq!(b, scale);
    }

    #[test]
    fn scale_negative_is_0() -> () {
        let c = Colour::new(-1.0, -2.0, -0.0);
        let (r, g, b) = c.scale(255);
        assert_eq!(r, 0);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn mul_colour() -> () {
        let c1 = Colour::new(1.0, 0.2, 0.4);
        let c2 = Colour::new(0.9, 1.0, 0.25);
        assert_eq!(c1 * c2, Colour::new(0.9, 0.2, 0.1));
    }

    #[test]
    fn mul_colour_by_scalar() -> () {
        let c = Colour::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Colour::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn sub_colour() -> () {
        // not sure how .9 is a sum of power of 2
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.9, 0.1, 0.25);
        assert_eq!(c1 - c2, Colour::new(0.0, 0.5, 0.5));
    }

    #[test]
    fn add_colour() -> () {
        let c1 = Colour::new(0.9, 0.6, 0.75);
        let c2 = Colour::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Colour::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn colour() -> () {
        let c = Colour {
            r: -0.5,
            g: 0.4,
            b: 1.7,
        };
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }
}
