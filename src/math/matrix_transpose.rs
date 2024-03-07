use std::ops::{Index, IndexMut};

use super::{matrix_const::MatrixConst, matrix_size::MatrixSize};

pub trait MatrixTranspose {
    fn transpose(&self) -> Self;
}

impl<
        T: MatrixConst + Index<(usize, usize)> + IndexMut<(usize, usize), Output = f64> + MatrixSize,
    > MatrixTranspose for T
{
    fn transpose(&self) -> Self {
        let mut res = Self::ID;
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                let v = self[(col, row)];
                res[(row, col)] = v;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::math::matrix::Matrix4x4;

    use super::*;

    #[test]
    fn transpose() -> () {
        let m = Matrix4x4::new(
            0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
        );
        let ex = Matrix4x4::new(
            0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
        );
        assert_eq!(m.transpose(), ex);
    }

    #[test]
    fn transpose_id() -> () {
        let id = Matrix4x4::ID;
        assert_eq!(id.transpose(), id);
    }
}
