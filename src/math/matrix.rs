use std::ops::Mul;

use super::{submatrix::SubMatrix, vector::Vector};

// todo: use nalgebra when done
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4x4 {
    m: [[f64; 4]; 4],
}

impl Matrix4x4 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> Self {
        Self {
            m: [[m00, m01, m02, m03], [m10, m11, m12, m13], [m20, m21, m22, m23], [m30, m31, m32, m33]],
        }
    }

    pub const ID: Matrix4x4 = Self {
        m: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
    };

    pub fn repeat(m: f64) -> Self {
        Self {
            m: [[m, m, m, m], [m, m, m, m], [m, m, m, m], [m, m, m, m]],
        }
    }

    pub fn transpose(&self) -> Self {
        let mut res = Matrix4x4::repeat(0.0);
        for row in 0..4 {
            for col in 0..4 {
                res.m[row][col] = self.m[col][row]
            }
        }
        res
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        let mut res = Matrix4x4::repeat(0.0);
        for row in 0..4 {
            for col in 0..4 {
                res.m[row][col] =
                    self.m[row][0] * rhs.m[0][col] +
                    self.m[row][1] * rhs.m[1][col] +
                    self.m[row][2] * rhs.m[2][col] +
                    self.m[row][3] * rhs.m[3][col]
            }
        }
        res
    }
}

impl Mul<Vector> for Matrix4x4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.m[0][0] * rhs.x + self.m[0][1] * rhs.y + self.m[0][2] * rhs.z + self.m[0][3] * rhs.w,
            y: self.m[1][0] * rhs.x + self.m[1][1] * rhs.y + self.m[1][2] * rhs.z + self.m[1][3] * rhs.w,
            z: self.m[2][0] * rhs.x + self.m[2][1] * rhs.y + self.m[2][2] * rhs.z + self.m[2][3] * rhs.w,
            w: self.m[3][0] * rhs.x + self.m[3][1] * rhs.y + self.m[3][2] * rhs.z + self.m[3][3] * rhs.w,
        }
    }
}

impl SubMatrix for Matrix4x4 {
    type Output = Matrix3x3;

    fn sub(&self, r: usize, c: usize) -> Self::Output {
        if r > 3 || c > 3 {
            Matrix3x3::repeat(0.0)
        } else {
            let mut v = Vec::with_capacity(9);
            for i in 0..4 {
                for j in 0..4 {
                    if i != r && j != c {
                        v.push(self.m[i][j]);
                    }
                }
            }
            Matrix3x3::from_iter(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub() -> () {
        let m = Matrix4x4::new(-6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0);
        let sub = m.sub(2, 1);
        let exp = Matrix3x3::new(-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0);
        assert_eq!(sub, exp);
    }

    #[test]
    fn transpose() -> () {
        let m = Matrix4x4::new(0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0);
        let ex = Matrix4x4::new(0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0);
        assert_eq!(m.transpose(), ex);
    }

    #[test]
    fn transpose_id() -> () {
        let id = Matrix4x4::ID;
        assert_eq!(id.transpose(), id);
    }

    #[test]
    fn id() -> () {
        let m = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let id = Matrix4x4::ID;
        assert_eq!(m * id, m);
        assert_eq!(id * m, m);
    }

    #[test]
    fn mul_vector() -> () {
        let m = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let v = Vector { x: 1.0, y: 2.0, z: 3.0, w: 1.0 };
        assert_eq!(m * v, Vector { x: 18.0, y: 24.0, z: 33.0, w: 1.0 });
    }

    #[test]
    fn mul() -> () {
        let m1 = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0);
        let m2 = Matrix4x4::new(-2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0);
        let ex = Matrix4x4::new(20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0, 46.0, 42.0);
        assert_eq!(m1 * m2, ex);
    }

    #[test]
    fn new() -> () {
        let m = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5);
        assert_eq!(m.m[0][0], 1.0);
        assert_eq!(m.m[0][3], 4.0);
        assert_eq!(m.m[1][0], 5.5);
        assert_eq!(m.m[1][2], 7.5);
        assert_eq!(m.m[2][2], 11.0);
        assert_eq!(m.m[3][0], 13.5);
        assert_eq!(m.m[3][2], 15.5);
    }

    #[test]
    fn repeat() -> () {
        let v = 2.22;
        let m = Matrix4x4::repeat(v);
        for row in m.m {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }

    #[test]
    fn eq() -> () {
        let v = 2.22;
        let m1 = Matrix4x4::repeat(v);
        let m2 = Matrix4x4::repeat(v);
        assert_eq!(m1, m2);
    }

    #[test]
    fn neq() -> () {
        let m1 = Matrix4x4::repeat(2.22);
        let m2 = Matrix4x4::repeat(2.21);
        assert_ne!(m1, m2);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2x2 {
    m: [[f64; 2]; 2],
}

impl Matrix2x2 {
    pub fn new(
        m00: f64,
        m01: f64,
        m10: f64,
        m11: f64,
    ) -> Self {
        Self {
            m: [[m00, m01], [m10, m11]],
        }
    }

    pub fn repeat(m: f64) -> Self {
        Self {
            m: [[m, m], [m, m]],
        }
    }

    pub fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let mut m = Self::repeat(0.0);
        let mut iter = items.into_iter();
        for i in 0..2 {
            for j in 0..2 {
                if let Some(item) = iter.next() {
                    m.m[i][j] = item;
                }
            }
        }
        m
    }

    pub fn det(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[1][0] * self.m[0][1]
    }
}

#[cfg(test)]
mod tests2x2 {
    use super::*;

    #[test]
    fn from_iter() -> () {
        let v1 = vec![0.0, 1.0, 2.0, 3.0];
        let m1 = Matrix2x2::from_iter(v1);
        let e1 = Matrix2x2::new(0.0, 1.0, 2.0, 3.0);
        assert_eq!(m1, e1);
        let v2 = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let m2 = Matrix2x2::from_iter(v2);
        let e2 = Matrix2x2::new(0.0, 1.0, 2.0, 3.0);
        assert_eq!(m2, e2);
        let v3 = vec![0.0, 1.0, 2.0];
        let m3 = Matrix2x2::from_iter(v3);
        let e3 = Matrix2x2::new(0.0, 1.0, 2.0, 0.0);
        assert_eq!(m3, e3);
    }

    #[test]
    fn det() -> () {
        let m = Matrix2x2::new(1.0, 5.0, -3.0, 2.0);
        assert_eq!(m.det(), 17.0);
    }

    #[test]
    fn new() -> () {
        let m = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);
        assert_eq!(m.m[0][0], -3.0);
        assert_eq!(m.m[0][1], 5.0);
        assert_eq!(m.m[1][0], 1.0);
        assert_eq!(m.m[1][1], -2.0);
    }

    #[test]
    fn repeat() -> () {
        let v = 2.22;
        let m = Matrix2x2::repeat(v);
        for row in m.m {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x3 {
    m: [[f64; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> Self {
        Self {
            m: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
        }
    }

    pub fn repeat(m: f64) -> Self {
        Self {
            m: [[m, m, m], [m, m, m], [m, m, m]],
        }
    }

    pub fn from_iter<I>(items: I) -> Self
    where
        I: IntoIterator<Item = f64>,
    {
        let mut m = Self::repeat(0.0);
        let mut iter = items.into_iter();
        for i in 0..3 {
            for j in 0..3 {
                if let Some(item) = iter.next() {
                    m.m[i][j] = item;
                }
            }
        }
        m
    }

    pub fn minor(&self, r: usize, c: usize) -> f64 {
        let sub = self.sub(r, c);
        sub.det()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        let minor = self.minor(r, c);
        if r + c % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl SubMatrix for Matrix3x3 {
    type Output = Matrix2x2;

    fn sub(&self, r: usize, c: usize) -> Self::Output {
        if r > 2 || c > 2 {
            Matrix2x2::repeat(0.0)
        } else {
            let mut v = Vec::with_capacity(4);
            for i in 0..3 {
                for j in 0..3 {
                    if i != r && j != c {
                        v.push(self.m[i][j]);
                    }
                }
            }
            Matrix2x2::from_iter(v)
        }
    }
}

#[cfg(test)]
mod tests3x3 {
    use super::*;

    #[test]
    fn cofactor() -> () {
        let m = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn minor() -> () {
        let m = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, 7.0, 6.0, -1.0, 5.0);
        assert_eq!(m.minor(1, 0), 25.0)
    }

    #[test]
    fn from_iter() -> () {
        let v1 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let m1 = Matrix3x3::from_iter(v1);
        let e1 = Matrix3x3::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        assert_eq!(m1, e1);
        let v2 = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let m2 = Matrix3x3::from_iter(v2);
        let e2 = Matrix3x3::new(0.0, 1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(m2, e2);
        let v3 = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let m3 = Matrix3x3::from_iter(v3);
        let e3 = Matrix3x3::new(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        assert_eq!(m3, e3);
    }

    #[test]
    fn sub() -> () {
        let m = Matrix3x3::new(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0);
        let sub = m.sub(0, 2);
        let exp = Matrix2x2::new(-3.0, 2.0, 0.0, 6.0);
        assert_eq!(sub, exp);
    }

    #[test]
    fn new() -> () {
        let m = Matrix3x3::new(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
        assert_eq!(m.m[0][0], -3.0);
        assert_eq!(m.m[1][1], -2.0);
        assert_eq!(m.m[2][2], 1.0);
    }

    #[test]
    fn repeat() -> () {
        let v = 2.22;
        let m = Matrix3x3::repeat(v);
        for row in m.m {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }
}