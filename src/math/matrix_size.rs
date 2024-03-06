use super::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};

pub trait MatrixSize {
    fn size(&self) -> usize;
}

impl MatrixSize for Matrix2x2 {
    fn size(&self) -> usize {
        2
    }
}

impl MatrixSize for Matrix3x3 {
    fn size(&self) -> usize {
        3
    }
}

impl MatrixSize for Matrix4x4 {
    fn size(&self) -> usize {
        4
    }
}