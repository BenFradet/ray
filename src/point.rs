use std::ops::{Add, Mul, Neg, Sub};

use crate::vector::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1.0 }
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
        Point { x: -self.x, y: -self.y, z: -self.z, w: -self.w, }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs, w: self.w * rhs, }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_for_point() -> () {
        let p = Point::new(1.0, -2.0, 3.0);
        let res = p * 0.5;
        assert_eq!(res, Point { x: 0.5, y: -1.0, z: 1.5, w: 0.5 })
    }

    #[test]
    fn neg_for_point() -> () {
        let p = Point::new(3.0, 2.0, 1.0);
        let res = -p;
        assert_eq!(res, Point { x: -3.0, y: -2.0, z: -1.0, w: -1.0 })
    }

    #[test]
    fn sub_vector_from_point() -> () {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        let res = p - v;
        assert_eq!(res, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn sub_point_from_point() -> () {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        let res = p1 - p2;
        assert_eq!(res, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn add_vector_to_point() -> () {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        let res = p + v;
        assert_eq!(res, Point::new(1.0, 1.0, 6.0));
    }
}