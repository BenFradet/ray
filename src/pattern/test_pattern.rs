use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TestPattern {}

impl PatternAt for TestPattern {
    fn pattern_at(&self, p: Point) -> Colour {
        Colour::new(p.x, p.y, p.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_at() -> () {
        let r = TestPattern {};
        assert_eq!(r.pattern_at(Point::ORIGIN), Colour::BLACK);
        assert_eq!(
            r.pattern_at(Point::new(1., 0., 0.)),
            Colour::new(1., 0., 0.)
        );
        assert_eq!(
            r.pattern_at(Point::new(0., 0., 1.)),
            Colour::new(0., 0., 1.)
        );
        assert_eq!(
            r.pattern_at(Point::new(0.708, 0., 0.708)),
            Colour::new(0.708, 0., 0.708)
        );
    }
}
