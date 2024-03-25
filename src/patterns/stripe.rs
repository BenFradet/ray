use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Stripe {
    a: Colour,
    b: Colour,
}

impl Stripe {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
    }
}

impl PatternAt for Stripe {
    fn pattern_at(&self, p: Point) -> Colour {
        if p.x.floor() % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stripe_varying_in_x() -> () {
        let s = Stripe::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(s.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(s.pattern_at(Point::new(0.9, 0., 0.)), Colour::WHITE);
        assert_eq!(s.pattern_at(Point::new(1., 0., 0.)), Colour::BLACK);
        assert_eq!(s.pattern_at(Point::new(-0.1, 0., 0.)), Colour::BLACK);
        assert_eq!(s.pattern_at(Point::new(-1., 0., 0.)), Colour::BLACK);
        assert_eq!(s.pattern_at(Point::new(-1.1, 0., 0.)), Colour::WHITE);
    }

    #[test]
    fn stripe_constant_in_z() -> () {
        let s = Stripe::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(s.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(s.pattern_at(Point::new(0., 0., 1.)), Colour::WHITE);
        assert_eq!(s.pattern_at(Point::new(0., 0., 2.)), Colour::WHITE);
    }

    #[test]
    fn stripe_constant_in_y() -> () {
        let s = Stripe::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(s.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(s.pattern_at(Point::new(0., 1., 0.)), Colour::WHITE);
        assert_eq!(s.pattern_at(Point::new(0., 2., 0.)), Colour::WHITE);
    }

    #[test]
    fn new() -> () {
        let s = Stripe::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(s.a, Colour::WHITE);
        assert_eq!(s.b, Colour::BLACK);
    }
}
