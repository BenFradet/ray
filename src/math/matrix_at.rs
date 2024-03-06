use super::{matrix::{Matrix2x2, Matrix3x3, Matrix4x4}, matrix_size::MatrixSize};

pub trait MatrixAt {
    fn at(&self, r: usize, c: usize) -> f64;
}

impl MatrixAt for Matrix2x2 {
    fn at(&self, r: usize, c: usize) -> f64 {
        if r >= self.size() || c >= self.size() {
            0.0
        } else {
            self.m[r][c]
        }
    }
}

impl MatrixAt for Matrix3x3 {
    fn at(&self, r: usize, c: usize) -> f64 {
        if r >= self.size() || c >= self.size() {
            0.0
        } else {
            self.m[r][c]
        }
    }
}

impl MatrixAt for Matrix4x4 {
    fn at(&self, r: usize, c: usize) -> f64 {
        if r >= self.size() || c >= self.size() {
            0.0
        } else {
            self.m[r][c]
        }
    }
}