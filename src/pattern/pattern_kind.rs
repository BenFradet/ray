use crate::math::{colour::Colour, point::Point};

use super::{gradient::Gradient, pattern_at::PatternAt, stripe::Stripe};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PatternKind {
    S(Stripe),
    G(Gradient),
}

impl PatternAt for PatternKind {
    fn pattern_at(&self, p: Point) -> Colour {
        match self {
            PatternKind::S(stripe) => stripe.pattern_at(p),
            PatternKind::G(gradient) => gradient.pattern_at(p),
        }
    }
}
