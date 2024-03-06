use super::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};

pub trait MatrixSize {
    const SIZE: usize;
}

impl MatrixSize for Matrix2x2 {
    const SIZE: usize = 2;
}

impl MatrixSize for Matrix3x3 {
    const SIZE: usize = 3;
}

impl MatrixSize for Matrix4x4 {
    const SIZE: usize = 4;
}