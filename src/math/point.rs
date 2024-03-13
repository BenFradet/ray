use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul, Neg, Sub},
};

use super::vector::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1. }
    }

    pub const ORIGIN: Point = Point::new(0., 0., 0.);
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "P({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl IntoIterator for Point {
    type Item = f64;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z, self.w].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_iter() -> () {
        let p = Point::new(1., -2., 3.);
        let exp = vec![1., -2., 3., 1.];
        assert!(p.into_iter().eq(exp));
    }

    #[test]
    fn mul_for_point() -> () {
        let p = Point::new(1., -2., 3.);
        let res = p * 0.5;
        assert_eq!(
            res,
            Point {
                x: 0.5,
                y: -1.,
                z: 1.5,
                w: 0.5
            }
        )
    }

    #[test]
    fn neg_for_point() -> () {
        let p = Point::new(3., 2., 1.);
        let res = -p;
        assert_eq!(
            res,
            Point {
                x: -3.,
                y: -2.,
                z: -1.,
                w: -1.
            }
        )
    }

    #[test]
    fn sub_vector_from_point() -> () {
        let p = Point::new(3., 2., 1.);
        let v = Vector::new(5., 6., 7.);
        let res = p - v;
        assert_eq!(res, Point::new(-2., -4., -6.));
    }

    #[test]
    fn sub_point_from_point() -> () {
        let p1 = Point::new(3., 2., 1.);
        let p2 = Point::new(5., 6., 7.);
        let res = p1 - p2;
        assert_eq!(res, Vector::new(-2., -4., -6.));
    }

    #[test]
    fn add_vector_to_point() -> () {
        let p = Point::new(3., -2., 5.);
        let v = Vector::new(-2., 3., 1.);
        let res = p + v;
        assert_eq!(res, Point::new(1., 1., 6.));
    }
}
