use crate::math::{point::Point, vector::Vector};

use super::{intersection::Intersection, ray::Ray};

#[derive(PartialEq, Debug, Clone)]
pub struct Comp<'a> {
    pub intersection: Intersection<'a>,
    pub point: Point,
    pub over_point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub reflect: Vector,
    pub inside: bool,
}

impl<'a> Comp<'a> {
    const EPS: f64 = 0.00001;

    pub fn new(intersection: Intersection<'a>, ray: Ray) -> Self {
        let point = ray.position(intersection.t);
        let eye = -ray.direction;
        let mut normal = intersection.shape.normal_at(point);
        let inside = if normal.dot(eye) < 0. {
            normal = -normal;
            true
        } else {
            false
        };
        let over_point = point + normal * Self::EPS;
        let reflect = ray.direction.reflect(normal);
        Self {
            intersection,
            point,
            over_point,
            eye,
            normal,
            reflect,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::{math::matrix::Matrix4x4, shape::shape::Shape};

    use super::*;

    #[test]
    fn reflect() -> () {
        let s = Shape::id_plane();
        let s2 = SQRT_2 / 2.;
        let r = Ray::new(Point::new(0., 1., -1.), Vector::new(0., -s2, s2));
        let i = Intersection::new(&s, s2 * 2.);
        let c = Comp::new(i, r);
        assert_eq!(c.reflect, Vector::new(0., s2, s2));
    }

    #[test]
    fn over_point() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::translation(0., 0., 1.)).unwrap_or(Shape::id_sphere());
        let i = Intersection::new(&s, 5.);
        let c = Comp::new(i, r);
        assert!(c.over_point.z < -Comp::EPS / 2.);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn inside() -> () {
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let s = Shape::id_sphere();
        let i = Intersection::new(&s, 1.);
        let c = Comp::new(i, r);
        assert_eq!(c.point, Point::new(0., 0., 1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
        assert!(c.inside);
    }

    #[test]
    fn not_inside() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::id_sphere();
        let i = Intersection::new(&s, 4.);
        let c = Comp::new(i, r);
        assert!(!c.inside);
    }

    #[test]
    fn new() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::id_sphere();
        let i = Intersection::new(&s, 4.);
        let it = i.t;
        let is = i.shape;
        let c = Comp::new(i, r);
        assert_eq!(c.intersection.t, it);
        assert_eq!(c.intersection.shape, is);
        assert_eq!(c.point, Point::new(0., 0., -1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
    }
}
