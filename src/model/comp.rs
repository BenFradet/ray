use crate::math::{point::Point, vector::Vector};

use super::{intersection::Intersection, ray::Ray};

pub struct Comp {
    intersection: Intersection,
    point: Point,
    eye: Vector,
    normal: Vector,
}

impl Comp {
    pub fn new(intersection: Intersection, ray: Ray) -> Self {
        let point = ray.position(intersection.t);
        Self {
            intersection,
            point,
            eye: -ray.direction,
            normal: intersection.object.normal_at(point),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::sphere::Sphere;

    use super::*;

    #[test]
    fn new() -> () {
        let r = Ray::new(Point::new(0., 0., -5.,), Vector::new(0., 0., 1.));
        let s = Sphere::id();
        let i = Intersection::new(4., s);
        let c = Comp::new(i, r);
        assert_eq!(c.intersection.t, i.t);
        assert_eq!(c.intersection.object, i.object);
        assert_eq!(c.point, Point::new(0., 0., -1.));
        assert_eq!(c.eye, Vector::new(0., 0., -1.));
        assert_eq!(c.normal, Vector::new(0., 0., -1.));
    }
}