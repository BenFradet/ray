use std::ops::IndexMut;

use super::{
    matrix_const::MatrixConst,
    matrix_det::{MatrixCofactor, MatrixDet},
    matrix_size::MatrixSize,
};

pub trait MatrixInvert {
    fn invert(&self) -> Option<Self>
    where
        Self: Sized;
}

impl<
        T: MatrixCofactor
            + MatrixDet
            + MatrixConst
            + MatrixSize
            + IndexMut<(usize, usize), Output = f64>,
    > MatrixInvert for T
{
    fn invert(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if !self.is_invertible() {
            None
        } else {
            let mut i = Self::ID;
            let det = self.det();
            for row in 0..Self::SIZE {
                for col in 0..Self::SIZE {
                    let c = self.cofactor(row, col);
                    let v = c / det;
                    i[(col, row)] = v;
                }
            }
            Some(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{
        matrix::{Matrix, Matrix4x4},
        matrix_transpose::MatrixTranspose, round::Round,
    };

    use super::*;

    #[test]
    fn transpose_invert() -> () {
        let a = Matrix4x4::new(
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        );
        let ati = a.transpose().invert();
        assert!(ati.is_some());
        let ait = a.invert().map(|m| m.transpose());
        assert!(ait.is_some());
        assert_eq!(ait, ati);
    }

    #[test]
    fn mul_invert() -> () {
        let a = Matrix4x4::new(
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        );
        let b = a.invert();
        assert!(b.is_some());
        assert_eq!(Matrix { m: (b.unwrap() * a) }.rounded(5), Matrix { m: Matrix4x4::ID }.rounded(5));
    }

    #[test]
    fn invert_id() -> () {
        let id = Matrix4x4::ID;
        let idi = id.invert();
        assert!(idi.is_some());
        assert_eq!(idi.unwrap(), id);
    }

    #[test]
    fn invert_mul() -> () {
        let a = Matrix4x4::new(
            3., -9., 7., 3., 3., -8., 2., -9., -4., 4., 4., 1., -6., 5., -1., 1.,
        );
        let b = Matrix4x4::new(
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        );
        let c = a * b;
        let bi = b.invert();
        assert!(bi.is_some());
        let biw = bi.unwrap();
        // round needed due to f64 repr
        let res = c * biw;
        assert_eq!(Matrix { m: res }.rounded(1), Matrix { m: a }.rounded(1));
    }

    #[test]
    fn invert3() -> () {
        let a = Matrix4x4::new(
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        );
        let b = a.invert();
        assert!(b.is_some());
        let bp = b.unwrap();
        let exp = vec![
            -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
            -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
        ];
        assert_eq!(Matrix { m: bp }.rounded(5), exp);
    }

    #[test]
    fn invert2() -> () {
        let a = Matrix4x4::new(
            8., -5., 9., 2., 7., 5., 6., 1., -6., 0., 9., 6., -3., 0., -9., -4.,
        );
        let b = a.invert();
        assert!(b.is_some());
        let bp = b.unwrap();
        let exp = vec![
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
            0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        ];
        assert_eq!(Matrix { m: bp }.rounded(5), exp);
    }

    #[test]
    fn invert1() -> () {
        let a = Matrix4x4::new(
            -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
        );
        let b = a.invert();
        assert_eq!(a.det(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);
        assert!(b.is_some());
        let bp = b.unwrap();
        assert_eq!(bp[(3, 2)], -160. / 532.);
        let exp = vec![
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
            -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        ];
        assert_eq!(Matrix { m: bp }.rounded(5), exp);
    }
}
