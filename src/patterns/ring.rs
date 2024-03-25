use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Ring {
    a: Colour,
    b: Colour,
}

impl Ring {
    pub fn new(a: Colour, b: Colour) -> Self {
        Self { a, b }
    }
}

impl PatternAt for Ring {
    fn pattern_at(&self, p: Point) -> Colour {
        let pxsq = p.x.powf(2.);
        let pzsq = p.z.powf(2.);
        let sum_sqrt = (pxsq + pzsq).sqrt();
        if sum_sqrt.floor() % 2. == 0. {
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
    fn pattern_at() -> () {
        let r = Ring::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(r.pattern_at(Point::ORIGIN), Colour::WHITE);
        assert_eq!(r.pattern_at(Point::new(1., 0., 0.)), Colour::BLACK);
        assert_eq!(r.pattern_at(Point::new(0., 0., 1.)), Colour::BLACK);
        assert_eq!(r.pattern_at(Point::new(0.708, 0., 0.708)), Colour::BLACK);
    }

    #[test]
    fn new() -> () {
        let r = Ring::new(Colour::WHITE, Colour::BLACK);
        assert_eq!(r.a, Colour::WHITE);
        assert_eq!(r.b, Colour::BLACK);
    }
}
