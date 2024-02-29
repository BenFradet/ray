use super::{point::Point, vector::Vector};

// haven't found a way to propagate enum meta's to variants because
// "meta-variable x repeats N times, but y repeats M times"
#[derive(PartialEq, Debug, Copy, Clone)]
enum Tuple {
    Vector(Vector),
    Point(Point),
}

impl Tuple {
    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::Vector(Vector::new(x, y, z))
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::Point(Point::new(x, y, z))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point() -> () {
        let p = Tuple::point(4.3, -4.2, 3.1);
        match p {
            Tuple::Point(Point { x, y, z, w }) => {
                assert_eq!(x, 4.3);
                assert_eq!(y, -4.2);
                assert_eq!(z, 3.1);
                assert_eq!(w, 1.0);
            }
            _ => panic!("not a point"),
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
            }
            _ => panic!("not a vector"),
        }
    }
}
