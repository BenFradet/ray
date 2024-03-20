use crate::math::{colour::Colour, point::Point};

use super::{
    checker::Checker, gradient::Gradient, pattern_at::PatternAt, radial_gradient::RadialGradient, ring::Ring, stripe::Stripe
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PatternKind {
    S(Stripe),
    G(Gradient),
    R(Ring),
    C(Checker),
    RG(RadialGradient),
}

impl PatternAt for PatternKind {
    fn pattern_at(&self, p: Point) -> Colour {
        match self {
            PatternKind::S(stripe) => stripe.pattern_at(p),
            PatternKind::G(gradient) => gradient.pattern_at(p),
            PatternKind::R(ring) => ring.pattern_at(p),
            PatternKind::C(checker) => checker.pattern_at(p),
            PatternKind::RG(radial_gradient) => radial_gradient.pattern_at(p),
        }
    }
}
