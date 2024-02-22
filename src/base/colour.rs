use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Colour {
    r: f64,
    g: f64,
    b: f64,
}

impl Colour {
    pub const fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b, }
    }

    pub const BLACK: Colour = Colour::new(0.0, 0.0, 0.0);
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
        let c = Colour { r: -0.5, g: 0.4, b: 1.7, };
        assert_eq!(c.r, -0.5);
        assert_eq!(c.g, 0.4);
        assert_eq!(c.b, 1.7);
    }
}