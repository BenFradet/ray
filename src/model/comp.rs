use crate::{
    math::{point::Point, vector::Vector},
    shape::{intersect::Intersect, normal::Normal},
};

use super::{intersection::Intersection, ray::Ray};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Comp<S: Intersect + Normal + Copy> {
    pub intersection: Intersection<S>,
    pub point: Point,
    pub over_point: Point,
    pub eye: Vector,
    pub normal: Vector,
    pub inside: bool,
}

impl<S: Intersect + Normal + Copy> Comp<S> {
    const EPS: f64 = 0.00001;

    pub fn new(intersection: Intersection<S>, ray: Ray) -> Self {
        let point = ray.position(intersection.t);
        let eye = -ray.direction;
        let mut normal = intersection.object.normal_at(point);
        let inside = if normal.dot(eye) < 0. {
            normal = -normal;
            true
        } else {
            false
        };
        let over_point = point + normal * Self::EPS;
        Self {
            intersection,
            point,
            over_point,
            eye,
            normal,
            inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::matrix::Matrix4x4,
        shape::{shape::Shape, sphere::Sphere},
    };

    use super::*;

    #[test]
    fn over_point() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new(Sphere {}, Matrix4x4::translation(0., 0., 1.))
            .unwrap_or(Shape::id(Sphere {}));
        let i = Intersection::new(5., s);
        let c = Comp::new(i, r);
        assert!(c.over_point.z < -Comp::<Sphere>::EPS / 2.);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn inside() -> () {
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        let s = Shape::id(Sphere {});
        let i = Intersection::new(1., s);
        let c = Comp::new(i, r);
        assert_eq!(c.point, Point::new(0., 0., 1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
        assert!(c.inside);
    }

    #[test]
    fn not_inside() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::id(Sphere {});
        let i = Intersection::new(4., s);
        let c = Comp::new(i, r);
        assert!(!c.inside);
    }

    #[test]
    fn new() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::id(Sphere {});
        let i = Intersection::new(4., s);
        let c = Comp::new(i, r);
        assert_eq!(c.intersection.t, i.t);
        assert_eq!(c.intersection.object, i.object);
        assert_eq!(c.point, Point::new(0., 0., -1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
    }
}
