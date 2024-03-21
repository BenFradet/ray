use crate::math::{colour::Colour, point::Point};

use super::pattern_at::PatternAt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Solid {
    c: Colour,
}

impl Solid {
    pub fn new(c: Colour) -> Self {
        Self { c }
    }
}

impl PatternAt for Solid {
    fn pattern_at(&self, _p: Point) -> Colour {
        self.c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_at() -> () {
        let s = Solid::new(Colour::WHITE);
        assert_eq!(s.pattern_at(Point::ORIGIN), Colour::WHITE);
    }

    #[test]
    fn new() -> () {
        let s = Solid::new(Colour::WHITE);
        assert_eq!(s.c, Colour::WHITE);
    }
}
