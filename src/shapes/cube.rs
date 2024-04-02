use std::f64::INFINITY;

use crate::model::ray::Ray;

use super::intersect::Intersect;

pub struct Cube {}

impl Cube {
    const EPS: f64 = 0.00001;
}

impl Intersect for Cube {
    fn intersect(&self, r: &Ray) -> Vec<f64> {
        fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
            let mut tmin = -1. - origin;
            let mut tmax = 1. - origin;

            if direction.abs() >= Cube::EPS {
                tmin /= direction;
                tmax /= direction;
            } else {
                tmin *= INFINITY;
                tmax *= INFINITY;
            }

            if tmin > tmax {
                (tmax, tmin)
            } else {
                (tmin, tmax)
            }
        }

        let (xtmin, xtmax) = check_axis(r.origin.x, r.direction.x);
        let (ytmin, ytmax) = check_axis(r.origin.y, r.direction.y);
        let (ztmin, ztmax) = check_axis(r.origin.z, r.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);
        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn no_intersect() -> () {
        vec![
            (Point::new(-2., 0., 0.), Vector::new(0.2673, 0.5345, 0.8018)),
            (Point::new(0., -2., 0.), Vector::new(0.8018, 0.2673, 0.5345)),
            (Point::new(0., 0., -2.), Vector::new(0.5345, 0.8018, 0.2673)),
            (Point::new(2., 0., 2.), Vector::new(0., 0., -1.)),
            (Point::new(0., 2., 2.), Vector::new(0., -1., 0.)),
            (Point::new(2., 2., 0.), Vector::new(-1., 0., 0.)),
        ]
        .iter()
        .for_each(|(origin, direction)| {
            let c = Cube {};
            let r = Ray::new(*origin, *direction);
            let res = c.intersect(&r);
            assert_eq!(res, vec![]);
        });
    }

    #[test]
    fn intersect() -> () {
        vec![
            (Point::new(5., 0.5, 0.), Vector::new(-1., 0., 0.), 4., 6.),
            (Point::new(-5., 0.5, 0.), Vector::new(1., 0., 0.), 4., 6.),
            (Point::new(0.5, 5., 0.), Vector::new(0., -1., 0.), 4., 6.),
            (Point::new(0.5, -5., 0.), Vector::new(0., 1., 0.), 4., 6.),
            (Point::new(0.5, 0., 5.), Vector::new(0., 0., -1.), 4., 6.),
            (Point::new(0.5, 0., -5.), Vector::new(0., 0., 1.), 4., 6.),
            (Point::new(0., 0.5, 0.), Vector::new(0., 0., 1.), -1., 1.),
        ]
        .iter()
        .for_each(|(origin, direction, t1, t2)| {
            let c = Cube {};
            let r = Ray::new(*origin, *direction);
            let res = c.intersect(&r);
            assert_eq!(res.len(), 2);
            assert_eq!(res[0], *t1);
            assert_eq!(res[1], *t2);
        });
    }
}
