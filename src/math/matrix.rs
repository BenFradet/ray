// todo: use nalgebra when done
#[derive(Debug, PartialEq)]
pub struct Matrix2x2 {
    storage: [[f64; 2]; 2],
}

impl Matrix2x2 {
    pub fn new(
        m00: f64,
        m01: f64,
        m10: f64,
        m11: f64,
    ) -> Self {
        Self {
            storage: [[m00, m01], [m10, m11]],
        }
    }

    pub fn repeat(m: f64) -> Self {
        Self {
            storage: [[m, m], [m, m]],
        }
    }
}

#[cfg(test)]
mod tests2x2 {
    use super::*;

    #[test]
    fn new() -> () {
        let m = Matrix2x2::new(-3.0, 5.0, 1.0, -2.0);
        assert_eq!(m.storage[0][0], -3.0);
        assert_eq!(m.storage[0][1], 5.0);
        assert_eq!(m.storage[1][0], 1.0);
        assert_eq!(m.storage[1][1], -2.0);
    }

    #[test]
    fn repeat() -> () {
        let v = 2.22;
        let m = Matrix2x2::repeat(v);
        for row in m.storage {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix3x3 {
    storage: [[f64; 3]; 3],
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
            storage: [[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]],
        }
    }

    pub fn repeat(m: f64) -> Self {
        Self {
            storage: [[m, m, m], [m, m, m], [m, m, m]],
        }
    }
}

#[cfg(test)]
mod tests3x3 {
    use super::*;

    #[test]
    fn new() -> () {
        let m = Matrix3x3::new(-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0);
        assert_eq!(m.storage[0][0], -3.0);
        assert_eq!(m.storage[1][1], -2.0);
        assert_eq!(m.storage[2][2], 1.0);
    }

    #[test]
    fn repeat() -> () {
        let v = 2.22;
        let m = Matrix3x3::repeat(v);
        for row in m.storage {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix4x4 {
    storage: [[f64; 4]; 4],
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
            storage: [[m00, m01, m02, m03], [m10, m11, m12, m13], [m20, m21, m22, m23], [m30, m31, m32, m33]],
        }
    }

    pub fn repeat(m: f64) -> Self {
        Self {
            storage: [[m, m, m, m], [m, m, m, m], [m, m, m, m], [m, m, m, m]],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() -> () {
        let m = Matrix4x4::new(1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5);
        assert_eq!(m.storage[0][0], 1.0);
        assert_eq!(m.storage[0][3], 4.0);
        assert_eq!(m.storage[1][0], 5.5);
        assert_eq!(m.storage[1][2], 7.5);
        assert_eq!(m.storage[2][2], 11.0);
        assert_eq!(m.storage[3][0], 13.5);
        assert_eq!(m.storage[3][2], 15.5);
    }

    #[test]
    fn repeat() -> () {
        let v = 2.22;
        let m = Matrix4x4::repeat(v);
        for row in m.storage {
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