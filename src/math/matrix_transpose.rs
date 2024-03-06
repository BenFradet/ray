use super::{matrix_from_iter::MatrixId, matrix_indexing::MatrixIndexing, matrix_size::MatrixSize};

pub trait MatrixTranspose {
    fn transpose(&self) -> Self;
}

impl <T: MatrixId + MatrixIndexing + MatrixSize> MatrixTranspose for T {
    fn transpose(&self) -> Self {
        let mut res = Self::ID;
        for row in 0..Self::SIZE {
            for col in 0..4 {
                let v = self.at(col, row);
                res.update_at(row, col, v);
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
        let m = Matrix4x4::new(0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0);
        let ex = Matrix4x4::new(0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0);
        assert_eq!(m.transpose(), ex);
    }

    #[test]
    fn transpose_id() -> () {
        let id = Matrix4x4::ID;
        assert_eq!(id.transpose(), id);
    }
}