use crate::math::{colour::Colour, point::Point};

use super::{
    checker::Checker, gradient::Gradient, nested::Nested, pattern_at::PatternAt, perlin::Perlin,
    radial_gradient::RadialGradient, ring::Ring, solid::Solid, stripe::Stripe,
    test_pattern::TestPattern,
};

#[derive(PartialEq, Debug, Clone)]
pub enum PatternKind {
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
    RadialGradient(RadialGradient),
    Solid(Solid),
    Nested(Box<Nested>),
    Perlin(Box<Perlin>),
    Test,
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
            PatternKind::Nested(nested) => nested.pattern_at(p),
            PatternKind::Perlin(perlin) => perlin.pattern_at(p),
            PatternKind::Test => TestPattern {}.pattern_at(p),
        }
    }
}
