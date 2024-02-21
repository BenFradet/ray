use std::ops::{Add, Sub};

use ray::enum_variants_as_structs;

// haven't found a way to propagate enum meta's to variants because
// "meta-variable x repeats N times, but y repeats M times"
enum_variants_as_structs! {
    #[derive(PartialEq, Debug, Copy, Clone)]
    enum Tuple {
        #[derive(PartialEq, Debug, Copy, Clone)]
        Vector { x: f32, y: f32, z: f32, w: f32 },
        #[derive(PartialEq, Debug, Copy, Clone)]
        Point { x: f32, y: f32, z: f32, w: f32 },
    }
}

impl Tuple {
    pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::Vector(Vector::new(x, y, z))
    }

    pub fn point(x: f32, y: f32, z: f32) -> Tuple {
        Tuple::Point(Point::new(x, y, z))
    }
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z, w: 1.0 }
    }
}

//impl Add<Tuple> for Tuple {
//    type Output = Tuple;
//
//    fn add(self, rhs: Tuple) -> Self::Output {
//        match self {
//            Tuple::Point(p) => {
//                match rhs {
//                    Tuple::Vector(v) => Tuple::Point(p + v),
//                    // adding a point to a point doesn't make sense
//                    // however since enum variants are not type we can't fine-grain add definitions
//                    o => o,
//                }
//            },
//            Tuple::Vector(v) => {
//                match rhs {
//                    Tuple::Vector(v2) => Tuple::Vector(v + v2),
//                    Tuple::Point(p) => Tuple::Point(p + v),
//                }
//            },
//        }
//    }
//}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn add_vector_to_vector() -> () {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let res = v1 + v2;
        assert_eq!(res, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn point() -> () {
        let p = Tuple::point(4.3, -4.2, 3.1);
        match p {
            Tuple::Point(Point { x, y, z, w }) => {
                assert_eq!(x, 4.3);
                assert_eq!(y, -4.2);
                assert_eq!(z, 3.1);
                assert_eq!(w, 1.0);
            },
            _ => panic!("not a point")
        }
    }

    #[test]
    fn vector() -> () {
        let v = Tuple::vector(4.3, -4.2, 3.1);
        match v {
            Tuple::Vector(Vector { x, y, z, w }) => {
                assert_eq!(x, 4.3);
                assert_eq!(y, -4.2);
                assert_eq!(z, 3.1);
                assert_eq!(w, 0.0);
            },
            _ => panic!("not a vector")
        }
    }
}