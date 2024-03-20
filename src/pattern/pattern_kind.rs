use crate::math::{colour::Colour, point::Point};

use super::{gradient::Gradient, pattern_at::PatternAt, ring::Ring, stripe::Stripe};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PatternKind {
    S(Stripe),
    G(Gradient),
    R(Ring),
}

impl PatternAt for PatternKind {
    fn pattern_at(&self, p: Point) -> Colour {
        match self {
            PatternKind::S(stripe) => stripe.pattern_at(p),
            PatternKind::G(gradient) => gradient.pattern_at(p),
            PatternKind::R(ring) => ring.pattern_at(p),
        }
    }
}
