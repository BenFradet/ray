use std::rc::Rc;

use crate::{
    math::{
        colour::Colour, matrix::Matrix4x4, matrix_const::MatrixConst, matrix_invert::MatrixInvert,
        point::Point,
    },
    shapes::shape::Shape,
};

use super::{
    checker::Checker, gradient::Gradient, pattern_at::PatternAt, pattern_kind::PatternKind,
    radial_gradient::RadialGradient, ring::Ring, stripe::Stripe,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Pattern {
    t: Matrix4x4,
    pub inv_t: Matrix4x4,
    pub underlying: PatternKind,
}

impl Pattern {
    pub fn new(p: PatternKind, t: Matrix4x4) -> Option<Self> {
        let inv = t.invert();
        inv.map(|inv_t| Self {
            t,
            inv_t,
            underlying: p,
        })
    }

    pub fn new_stripe(a: Colour, b: Colour, t: Matrix4x4) -> Option<Self> {
        Self::new(PatternKind::Stripe(Stripe::new(a, b)), t)
    }

    pub fn new_gradient(a: Colour, b: Colour, t: Matrix4x4) -> Option<Self> {
        Self::new(PatternKind::Gradient(Gradient::new(a, b)), t)
    }

    pub fn new_radial_gradient(a: Colour, b: Colour, t: Matrix4x4) -> Option<Self> {
        Self::new(PatternKind::RadialGradient(RadialGradient::new(a, b)), t)
    }

    pub fn new_ring(a: Colour, b: Colour, t: Matrix4x4) -> Option<Self> {
        Self::new(PatternKind::Ring(Ring::new(a, b)), t)
    }

    pub fn new_checker(a: Colour, b: Colour, t: Matrix4x4) -> Option<Self> {
        Self::new(PatternKind::Checker(Checker::new(a, b)), t)
    }

    pub fn id(p: PatternKind) -> Self {
        Self {
            t: Matrix4x4::ID,
            inv_t: Matrix4x4::ID,
            underlying: p,
        }
    }

    pub fn id_stripe(a: Colour, b: Colour) -> Self {
        Self::id(PatternKind::Stripe(Stripe::new(a, b)))
    }

    pub fn id_gradient(a: Colour, b: Colour) -> Self {
        Self::id(PatternKind::Gradient(Gradient::new(a, b)))
    }

    pub fn id_radial_gradient(a: Colour, b: Colour) -> Self {
        Self::id(PatternKind::RadialGradient(RadialGradient::new(a, b)))
    }

    pub fn id_ring(a: Colour, b: Colour) -> Self {
        Self::id(PatternKind::Ring(Ring::new(a, b)))
    }

    pub fn id_checker(a: Colour, b: Colour) -> Self {
        Self::id(PatternKind::Checker(Checker::new(a, b)))
    }

    pub fn at_shape(&self, s: Rc<Shape>, world_p: Point) -> Colour {
        let object_p = s.inv_t * world_p;
        let pattern_p = self.inv_t * object_p;
        self.underlying.pattern_at(pattern_p)
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::colour::Colour, patterns::stripe::Stripe};

    use super::*;

    #[test]
    fn at_shape_pattern_shape_ts() -> () {
        let s = Rc::new(Shape::new_sphere(Matrix4x4::scaling(2., 2., 2.)).unwrap());
        let p = Pattern::new_stripe(
            Colour::WHITE,
            Colour::BLACK,
            Matrix4x4::translation(0.5, 0., 0.),
        )
        .unwrap();
        let res = p.at_shape(s, Point::new(2.5, 0., 0.));
        assert_eq!(res, Colour::WHITE);
    }

    #[test]
    fn at_shape_pattern_t() -> () {
        let s = Rc::new(Shape::id_sphere());
        let p = Pattern::new_stripe(Colour::WHITE, Colour::BLACK, Matrix4x4::scaling(2., 2., 2.))
            .unwrap();
        let res = p.at_shape(s, Point::new(1.5, 0., 0.));
        assert_eq!(res, Colour::WHITE);
    }

    #[test]
    fn at_shape_shape_t() -> () {
        let s = Rc::new(Shape::new_sphere(Matrix4x4::scaling(2., 2., 2.)).unwrap());
        let p = Pattern::id_stripe(Colour::WHITE, Colour::BLACK);
        let res = p.at_shape(s, Point::new(1.5, 0., 0.));
        assert_eq!(res, Colour::WHITE);
    }

    #[test]
    fn id() -> () {
        let pk = PatternKind::Stripe(Stripe::new(Colour::WHITE, Colour::BLACK));
        let p = Pattern::id(pk.clone());
        assert_eq!(p.t, Matrix4x4::ID);
        assert_eq!(p.inv_t, Matrix4x4::ID);
        assert_eq!(p.underlying, pk);
    }

    #[test]
    fn new() -> () {
        let pk = PatternKind::Stripe(Stripe::new(Colour::WHITE, Colour::BLACK));
        let p = Pattern::new(pk.clone(), Matrix4x4::translation(1., 0., 0.));
        assert!(p.is_some());
        let pp = p.unwrap();
        assert_eq!(pp.t, Matrix4x4::translation(1., 0., 0.));
        let mut exp = Matrix4x4::ID;
        exp[(0, 3)] = -1.;
        assert_eq!(pp.inv_t, exp);
        assert_eq!(pp.underlying, pk);
    }
}
