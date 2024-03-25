use std::ops::Index;

use super::{matrix::Matrix2x2, matrix_size::MatrixSize, matrix_sub::MatrixSub};

pub trait MatrixCofactor {
    fn minor(&self, r: usize, c: usize) -> f64;
    fn cofactor(&self, r: usize, c: usize) -> f64;
}

impl<A: MatrixSub<Output = B>, B: MatrixDet> MatrixCofactor for A {
    fn minor(&self, r: usize, c: usize) -> f64 {
        let sub = self.sub(r, c);
        sub.det()
    }

    fn cofactor(&self, r: usize, c: usize) -> f64 {
        let minor = self.minor(r, c);
        //let sign = (-1i32).pow(r as u32 + c as u32);
        //sign as f64 * minor
        if (r + c) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

pub trait MatrixDet {
    fn det(&self) -> f64;
    fn is_invertible(&self) -> bool;
}

impl<A: MatrixCofactor + MatrixSize + Index<(usize, usize), Output = f64>> MatrixDet for A {
    fn det(&self) -> f64 {
        let mut det = 0.;
        for i in 0..Self::SIZE {
            det += self[(0, i)] * self.cofactor(0, i);
        }
        det
    }

    fn is_invertible(&self) -> bool {
        self.det() != 0.
    }
}

impl MatrixDet for Matrix2x2 {
    fn det(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[1][0] * self.m[0][1]
    }

    fn is_invertible(&self) -> bool {
        self.det() != 0.
    }
}

#[cfg(test)]
mod tests {
    use crate::math::matrix::{Matrix3x3, Matrix4x4};

    use super::*;

    #[test]
    fn is_invertible() -> () {
        let m = Matrix4x4::new(
            6., 4., 4., 4., 5., 5., 7., 6., 4., -8., 3., -7., 9., 1., 7., -6.,
        );
        assert!(m.is_invertible())
    }

    #[test]
    fn is_not_invertible() -> () {
        let m = Matrix4x4::new(
            -4., 2., -2., 3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
        );
        assert!(!m.is_invertible())
    }

    #[test]
    fn det4x4() -> () {
        let m = Matrix4x4::new(
            -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
        );
        assert_eq!(m.cofactor(0, 0), 690.);
        assert_eq!(m.cofactor(0, 1), 447.);
        assert_eq!(m.cofactor(0, 2), 210.);
        assert_eq!(m.cofactor(0, 3), 51.);
        assert_eq!(m.det(), -4071.);
    }

    #[test]
    fn det3x3() -> () {
        let m = Matrix3x3::new(1., 2., 6., -5., 8., -4., 2., 6., 4.);
        assert_eq!(m.cofactor(0, 0), 56.);
        assert_eq!(m.cofactor(0, 1), 12.);
        assert_eq!(m.cofactor(0, 2), -46.);
        assert_eq!(m.det(), -196.);
    }

    #[test]
    fn det2x2() -> () {
        let m = Matrix2x2::new(1., 5., -3., 2.);
        assert_eq!(m.det(), 17.);
    }

    #[test]
    fn cofactor() -> () {
        let m = Matrix3x3::new(3., 5., 0., 2., -1., -7., 6., -1., 5.);
        assert_eq!(m.cofactor(0, 0), -12.);
        assert_eq!(m.cofactor(1, 0), -25.);
    }

    #[test]
    fn minor() -> () {
        let m = Matrix3x3::new(3., 5., 0., 2., -1., 7., 6., -1., 5.);
        assert_eq!(m.minor(1, 0), 25.)
    }
}
