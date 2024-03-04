use std::ops::Mul;

use super::vector::Vector;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}

#[cfg(test)]
mod tests2x2 {
    use super::*;

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
}

#[cfg(test)]
mod tests3x3 {
    use super::*;

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