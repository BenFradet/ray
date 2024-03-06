use super::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};


pub trait MatrixConst {
    const ID: Self;
    const ZERO: Self;
}

impl MatrixConst for Matrix4x4 {
    const ID: Self = Self {
        m: [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 1., 0.], [0., 0., 0., 1.]],
    };
    const ZERO: Self = Self {
        m: [[0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.], [0., 0., 0., 0.]],
    };
}

impl MatrixConst for Matrix3x3 {
    const ID: Self = Self {
        m: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    };
    const ZERO: Self = Self {
        m: [[0., 0., 0.], [0., 0., 0.], [0., 0., 0.]],
    };
}

impl MatrixConst for Matrix2x2 {
    const ID: Self = Self {
        m: [[1., 0.], [0., 1.]],
    };
    const ZERO: Self = Self {
        m: [[0., 0.], [0., 0.]],
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::math::matrix::Matrix4x4;

    #[test]
    fn id4x4() -> () {
        let m = Matrix4x4::new(1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.);
        let id = Matrix4x4::ID;
        assert_eq!(m * id, m);
        assert_eq!(id * m, m);
    }
}