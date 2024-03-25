use crate::{
    math::{point::Point, vector::Vector},
    model::ray::Ray,
};

use super::{intersect::Intersect, normal::Normal};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {}

impl Sphere {
    pub const RADIUS: f64 = 1.;
}

impl Normal for Sphere {
    fn normal_at(&self, object_point: Point) -> Vector {
        object_point - Point::ORIGIN
    }
}

impl Intersect for Sphere {
    // https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection
    fn intersect(&self, r: &Ray) -> Vec<f64> {
        let sphere_to_ray = r.origin - Point::ORIGIN;
        let a = r.direction.dot(r.direction);
        let b = 2. * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - Sphere::RADIUS.powf(2.);
        let discriminant = b.powf(2.) - 4. * a * c;

        if discriminant < 0. {
            vec![]
        } else {
            // discriminant = 0 is one solution but we still output two
            let t1 = (-b - discriminant.sqrt()) / (2. * a);
            let t2 = (-b + discriminant.sqrt()) / (2. * a);
            // prolly an issue here
            vec![t1, t2]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{math::matrix::Matrix4x4, model::intersection::Intersection, shapes::shape::Shape};

    use super::*;

    #[test]
    fn normal_at() -> () {
        let s = Sphere {};
        assert_eq!(s.normal_at(Point::new(1., 0., 0.)), Vector::new(1., 0., 0.));
        assert_eq!(s.normal_at(Point::new(0., 1., 0.)), Vector::new(0., 1., 0.));
        assert_eq!(s.normal_at(Point::new(0., 0., 1.)), Vector::new(0., 0., 1.));
        let s3 = 3f64.sqrt() / 3.;
        let res = s.normal_at(Point::new(s3, s3, s3));
        assert_eq!(res, Vector::new(s3, s3, s3));
        assert_eq!(res.norm(), res);
    }

    #[test]
    fn intersect_after_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::id_sphere());
        let res = Intersection::intersections(Rc::clone(&s), &r);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(Rc::clone(&s), -6.));
        assert_eq!(res[1], Intersection::new(Rc::clone(&s), -4.));
    }

    #[test]
    fn intersect_inside_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::id_sphere());
        let res = Intersection::intersections(Rc::clone(&s), &r);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(Rc::clone(&s), -1.));
        assert_eq!(res[1], Intersection::new(Rc::clone(&s), 1.));
    }

    #[test]
    fn intersect_no_points() -> () {
        let r = Ray::new(Point::new(0., 2., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::id_sphere());
        let res = Intersection::intersections(s, &r);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_same_point() -> () {
        let r = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::id_sphere());
        let res = Intersection::intersections(Rc::clone(&s), &r);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(Rc::clone(&s), 5.));
        assert_eq!(res[1], Intersection::new(Rc::clone(&s), 5.));
    }

    #[test]
    fn intersect_2_points() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::id_sphere());
        let res = Intersection::intersections(Rc::clone(&s), &r);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(Rc::clone(&s), 4.));
        assert_eq!(res[1], Intersection::new(Rc::clone(&s), 6.));
    }

    #[test]
    fn intersect_translated_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(
            Shape::id_sphere()
                .t(Matrix4x4::translation(5., 0., 0.))
                .unwrap(),
        );
        let res = Intersection::intersections(s, &r);
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn intersect_scaled_sphere() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Rc::new(Shape::new_sphere(Matrix4x4::scaling(2., 2., 2.)).unwrap());
        let res = Intersection::intersections(Rc::clone(&s), &r);
        assert_eq!(res.len(), 2);
        assert_eq!(res[0], Intersection::new(Rc::clone(&s), 3.));
        assert_eq!(res[1], Intersection::new(Rc::clone(&s), 7.));
    }
}
