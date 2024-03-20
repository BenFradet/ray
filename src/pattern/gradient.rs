use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Gradient {
    a: Colour,
    b: Colour,
}

impl Gradient {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
    }
}

impl PatternAt for Gradient {
    fn pattern_at(&self, p: Point) -> Colour {
        self.a + (self.b - self.a) * (p.x - p.x.floor())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_at() -> () {
        let g = Gradient::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(g.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(
            g.pattern_at(Point::new(0.25, 0., 0.)),
            Colour::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            g.pattern_at(Point::new(0.5, 0., 0.)),
            Colour::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            g.pattern_at(Point::new(0.75, 0., 0.)),
            Colour::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn new() -> () {
        let g = Gradient::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(g.a, Colour::WHITE);
        assert_eq!(g.b, Colour::BLACK);
    }
}
