use super::{matrix::{Matrix2x2, Matrix3x3, Matrix4x4}, matrix_size::MatrixSize};

pub trait MatrixIndexing {
    fn at(&self, r: usize, c: usize) -> f64;
    fn update_at(&mut self, r: usize, c: usize, e: f64) -> ();
}

impl MatrixIndexing for Matrix2x2 {
    fn at(&self, r: usize, c: usize) -> f64 {
        if r >= Self::SIZE || c >= Self::SIZE {
            0.0
        } else {
            self.m[r][c]
        }
    }

    fn update_at(&mut self, r: usize, c: usize, e: f64) -> () {
        if r < Self::SIZE || c < Self::SIZE {
            self.m[r][c] = e;
        }
    }
}

impl MatrixIndexing for Matrix3x3 {
    fn at(&self, r: usize, c: usize) -> f64 {
        if r >= Self::SIZE || c >= Self::SIZE {
            0.0
        } else {
            self.m[r][c]
        }
    }

    fn update_at(&mut self, r: usize, c: usize, e: f64) -> () {
        self.m[r][c] = e;
    }
}

impl MatrixIndexing for Matrix4x4 {
    fn at(&self, r: usize, c: usize) -> f64 {
        if r >= Self::SIZE || c >= Self::SIZE {
            0.0
        } else {
            self.m[r][c]
        }
    }

    fn update_at(&mut self, r: usize, c: usize, e: f64) -> () {
        self.m[r][c] = e;
    }
}