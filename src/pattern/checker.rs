use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Checker {
    a: Colour,
    b: Colour,
}

impl Checker {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
    }
}

impl PatternAt for Checker {
    fn pattern_at(&self, p: Point) -> Colour {
        if (p.x.floor() + p.y.floor() + p.z.floor()) % 2. == 0. {
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
    fn pattern_at_y() -> () {
        let c = Checker::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(c.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(c.pattern_at(Point::new(0., 0.99, 0.)), Colour::WHITE);
        assert_eq!(c.pattern_at(Point::new(0., 1.01, 0.)), Colour::BLACK);
    }
    #[test]
    fn pattern_at_z() -> () {
        let c = Checker::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(c.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(c.pattern_at(Point::new(0., 0., 0.99)), Colour::WHITE);
        assert_eq!(c.pattern_at(Point::new(0., 0., 1.01)), Colour::BLACK);
    }

    #[test]
    fn pattern_at_x() -> () {
        let c = Checker::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(c.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(c.pattern_at(Point::new(0.99, 0., 0.)), Colour::WHITE);
        assert_eq!(c.pattern_at(Point::new(1.01, 0., 0.)), Colour::BLACK);
    }

    #[test]
    fn new() -> () {
        let c = Checker::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(c.a, Colour::WHITE);
        assert_eq!(c.b, Colour::BLACK);
    }
}
