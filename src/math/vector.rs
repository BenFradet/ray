use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Mul, Neg, Sub},
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }

    pub fn len(self) -> f64 {
        f64::sqrt(self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0))
    }

    pub fn norm(self) -> Vector {
        let l = self.len();
        Vector {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
            w: self.w / l,
        }
    }

    pub fn dot(self, o: Vector) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }

    pub fn cross(self, o: Vector) -> Vector {
        Vector::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "V({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_cross() -> () {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn vec_dot() -> () {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let res = v1.dot(v2);
        assert_eq!(res, 20.0)
    }

    #[test]
    fn vec_norm() -> () {
        assert_eq!(
            Vector::new(4.0, 0.0, 0.0).norm(),
            Vector::new(1.0, 0.0, 0.0)
        );
        let v = Vector::new(1.0, 2.0, 3.0);
        let sqrt = f64::sqrt(14.0);
        let norm = v.norm();
        assert_eq!(norm, Vector::new(1.0 / sqrt, 2.0 / sqrt, 3.0 / sqrt));
        assert_eq!(norm.len(), 1.0);
    }

    #[test]
    fn vec_len() -> () {
        assert_eq!(Vector::new(1.0, 0.0, 0.0).len(), 1.0);
        assert_eq!(Vector::new(0.0, 1.0, 0.0).len(), 1.0);
        assert_eq!(Vector::new(0.0, 0.0, 1.0).len(), 1.0);
        assert_eq!(Vector::new(1.0, 2.0, 3.0).len(), f64::sqrt(14.0));
        assert_eq!(Vector::new(-1.0, -2.0, -3.0).len(), f64::sqrt(14.0));
    }

    #[test]
    fn mul_for_vec() -> () {
        let p = Vector::new(1.0, -2.0, 3.0);
        let res = p * 0.5;
        assert_eq!(res, Vector::new(0.5, -1.0, 1.5))
    }

    #[test]
    fn neg_for_vec() -> () {
        let v = Vector::new(3.0, 2.0, 1.0);
        let res = -v;
        assert_eq!(res, Vector::new(-3.0, -2.0, -1.0))
    }

    #[test]
    fn sub_vec_from_vec() -> () {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        let res = v1 - v2;
        assert_eq!(res, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn add_vector_to_vector() -> () {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let res = v1 + v2;
        assert_eq!(res, Vector::new(1.0, 1.0, 6.0));
    }
}
