use std::f64::INFINITY;

use crate::{
    math::{point::Point, vector::Vector},
    model::ray::Ray,
};

use super::{intersect::Intersect, normal::Normal};

pub struct Cube {}

impl Cube {
    const EPS: f64 = 0.00001;
}

impl Normal for Cube {
    fn normal_at(&self, object_point: Point) -> Vector {
        let maxc = object_point
            .x
            .abs()
            .max(object_point.y.abs())
            .max(object_point.z.abs());

        if maxc == object_point.x.abs() {
            Vector::new(object_point.x, 0., 0.)
        } else if maxc == object_point.y.abs() {
            Vector::new(0., object_point.y, 0.)
        } else {
            Vector::new(0., 0., object_point.z)
        }
    }
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
        if xtmin == xtmax && xtmin.abs() > 1. {
            return vec![];
        }
        let (ytmin, ytmax) = check_axis(r.origin.y, r.direction.y);
        if ytmin == ytmax && ytmin.abs() > 1. {
            return vec![];
        }
        if xtmin.abs() > 1.
            && ytmin.abs() > 1.
            && xtmax.abs() > 1.
            && ytmax.abs() > 1.
            && xtmin.signum() == xtmax.signum()
            && ytmin.signum() == ytmax.signum()
        {
            return vec![];
        }
        let (ztmin, ztmax) = check_axis(r.origin.z, r.direction.z);
        if ztmin == ztmax && ztmin.abs() > 1. {
            return vec![];
        }

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
    use super::*;

    #[test]
    fn normal_at() -> () {
        vec![
            (Point::new(1., 0.5, -0.8), Vector::new(1., 0., 0.)),
            (Point::new(-1., -0.2, 0.9), Vector::new(-1., 0., 0.)),
            (Point::new(-0.4, 1., -0.1), Vector::new(0., 1., 0.)),
            (Point::new(0.3, -1., -0.7), Vector::new(0., -1., 0.)),
            (Point::new(-0.6, 0.3, 1.), Vector::new(0., 0., 1.)),
            (Point::new(0.4, 0.4, -1.), Vector::new(0., 0., -1.)),
            (Point::new(1., 1., 1.), Vector::new(1., 0., 0.)),
            (Point::new(-1., -1., -1.), Vector::new(-1., 0., 0.)),
        ]
        .iter()
        .for_each(|(point, normal)| {
            let c = Cube {};
            let res = c.normal_at(*point);
            assert_eq!(res, *normal);
        });
    }

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
