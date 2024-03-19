use crate::{math::{point::Point, vector::Vector}, model::ray::Ray};

use super::{intersect::Intersect, normal::Normal};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Plane {}

impl Plane {
    const EPS: f64 = 0.00001;
}

impl Normal for Plane {
    fn normal_at(&self, _object_point: Point) -> Vector {
        Vector::new(0., 1., 0.)
    }
}

impl Intersect for Plane {
    fn intersect(&self, r: &Ray) -> Vec<f64> {
        if r.direction.y.abs() < Plane::EPS {
            vec![]
        } else {
            let t = -r.origin.y / r.direction.y;
            vec![t]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_from_below() -> () {
        let p = Plane {};
        let r = Ray::new(Point::new(0., -2., 0.), Vector::new(0., 1., 0.));
        assert_eq!(p.intersect(&r), vec![2.])
    }

    #[test]
    fn intersect_from_above() -> () {
        let p = Plane {};
        let r = Ray::new(Point::new(2., 2., 2.), Vector::new(0., -1., 0.));
        assert_eq!(p.intersect(&r), vec![2.])
    }

    #[test]
    fn intersect_coplanar() -> () {
        let p = Plane {};
        let r = Ray::new(Point::ORIGIN, Vector::new(0., 0., 1.));
        assert_eq!(p.intersect(&r), Vec::<f64>::new()); 
    }

    #[test]
    fn intersect_parallel() -> () {
        let p = Plane {};
        let r = Ray::new(Point::new(0., 10., 0.), Vector::new(0., 0., 1.));
        assert_eq!(p.intersect(&r), Vec::<f64>::new());
    }

    #[test]
    fn constant_normal() -> () {
        let p = Plane {};
        let exp = Vector::new(0., 1., 0.);
        assert_eq!(p.normal_at(Point::ORIGIN), exp);
        assert_eq!(p.normal_at(Point::new(10., 0., -10.)), exp);
        assert_eq!(p.normal_at(Point::new(-5., 0., 150.)), exp);
    }
}