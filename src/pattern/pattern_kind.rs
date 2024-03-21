use crate::math::{colour::Colour, point::Point};

use super::{
    checker::Checker, gradient::Gradient, pattern_at::PatternAt, radial_gradient::RadialGradient,
    ring::Ring, solid::Solid, stripe::Stripe,
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PatternKind {
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
    RadialGradient(RadialGradient),
    Solid(Solid),
    //Nested(Box<Nested>),
}

impl PatternAt for PatternKind {
    fn pattern_at(&self, p: Point) -> Colour {
        match self {
            PatternKind::Stripe(stripe) => stripe.pattern_at(p),
            PatternKind::Gradient(gradient) => gradient.pattern_at(p),
            PatternKind::Ring(ring) => ring.pattern_at(p),
            PatternKind::Checker(checker) => checker.pattern_at(p),
            PatternKind::RadialGradient(radial_gradient) => radial_gradient.pattern_at(p),
            PatternKind::Solid(solid) => solid.pattern_at(p),
            //PatternKind::Nested(nested_box) => nested_box.pattern_at(p),
        }
    }
}
