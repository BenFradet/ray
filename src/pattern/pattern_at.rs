use crate::math::{colour::Colour, point::Point};

pub trait PatternAt {
    fn pattern_at(&self, p: Point) -> Colour;
}
