use super::matrix::{Matrix2x2, Matrix3x3, Matrix4x4};

pub trait SubMatrix {
    type Output;

    fn sub(&self, r: usize, c: usize) -> Self::Output;
}

impl SubMatrix for Matrix3x3 {
    type Output = Matrix2x2;

    fn sub(&self, r: usize, c: usize) -> Self::Output {
        if r > 2 || c > 2 {
            Matrix2x2::repeat(0.0)
        } else {
            let mut v = Vec::with_capacity(4);
            for i in 0..3 {
                for j in 0..3 {
                    if i != r && j != c {
                        v.push(self.m[i][j]);
                    }
                }
            }
            Matrix2x2::from_iter(v)
        }
    }
}

impl SubMatrix for Matrix4x4 {
    type Output = Matrix3x3;

    fn sub(&self, r: usize, c: usize) -> Self::Output {
        if r > 3 || c > 3 {
            Matrix3x3::repeat(0.0)
        } else {
            let mut v = Vec::with_capacity(9);
            for i in 0..4 {
                for j in 0..4 {
                    if i != r && j != c {
                        v.push(self.m[i][j]);
                    }
                }
            }
            Matrix3x3::from_iter(v)
        }
    }
}