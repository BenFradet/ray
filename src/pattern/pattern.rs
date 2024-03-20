use crate::math::{colour::Colour, point::Point};

use super::{pattern_at::PatternAt, stripe::Stripe};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Pattern {
    S(Stripe),
}

impl PatternAt for Pattern {
    fn pattern_at(&self, p: Point) -> Colour {
        match self {
            Pattern::S(stripe) => stripe.pattern_at(p),
        }
    }
}