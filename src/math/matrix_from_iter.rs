use std::iter;

use super::{matrix::{Matrix2x2, Matrix3x3, Matrix4x4}, matrix_const::MatrixConst, matrix_indexing::MatrixIndexing, matrix_size::MatrixSize};

pub trait MatrixFromIter {
    fn from_iter<I>(items: I) -> Self where I: IntoIterator<Item = f64>;
    fn repeat(m: f64) -> Self;
}

impl <T: MatrixConst + MatrixSize + MatrixIndexing> MatrixFromIter for T {
    fn from_iter<I>(items: I) -> Self where I: IntoIterator<Item = f64> {
        let mut m = Self::ZERO;
        let mut iter = items.into_iter();
        for i in 0..Self::SIZE {
            for j in 0..Self::SIZE {
                if let Some(item) = iter.next() {
                    m.update_at(i, j, item);
                }
            }
        }
        m
    }

    fn repeat(m: f64) -> Self {
        let i = iter::repeat(m);
        Self::from_iter(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn repeat_3x3() -> () {
        let v = 2.22;
        let m = Matrix3x3::repeat(v);
        for row in m.m {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }

    #[test]
    fn from_iter_3x3() -> () {
        let v1 = vec![0., 1., 2., 3., 4., 5., 6., 7., 8.];
        let m1 = Matrix3x3::from_iter(v1);
        let e1 = Matrix3x3::new(0., 1., 2., 3., 4., 5., 6., 7., 8.);
        assert_eq!(m1, e1);
        let v2 = vec![0., 1., 2., 3., 4.];
        let m2 = Matrix3x3::from_iter(v2);
        let e2 = Matrix3x3::new(0., 1., 2., 3., 4., 0., 0., 0., 0.);
        assert_eq!(m2, e2);
        let v3 = vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
        let m3 = Matrix3x3::from_iter(v3);
        let e3 = Matrix3x3::new(0., 1., 2., 3., 4., 5., 6., 7., 8.);
        assert_eq!(m3, e3);
    }

    #[test]
    fn repeat_2x2() -> () {
        let v = 2.22;
        let m = Matrix2x2::repeat(v);
        for row in m.m {
            for i in row {
                assert_eq!(i, v);
            }
        }
    }

    #[test]
    fn from_iter_2x2() -> () {
        let v1 = vec![0., 1., 2., 3.];
        let m1 = Matrix2x2::from_iter(v1);
        let e1 = Matrix2x2::new(0., 1., 2., 3.);
        assert_eq!(m1, e1);
        let v2 = vec![0., 1., 2., 3., 4.];
        let m2 = Matrix2x2::from_iter(v2);
        let e2 = Matrix2x2::new(0., 1., 2., 3.);
        assert_eq!(m2, e2);
        let v3 = vec![0., 1., 2.];
        let m3 = Matrix2x2::from_iter(v3);
        let e3 = Matrix2x2::new(0., 1., 2., 0.);
        assert_eq!(m3, e3);
    }
}