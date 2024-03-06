use super::{matrix::{Matrix2x2, Matrix3x3, Matrix4x4}, matrix_from_iter::MatrixFromIter};

pub trait MatrixSub {
    type Output;

    fn sub(&self, r: usize, c: usize) -> Self::Output;
}

impl MatrixSub for Matrix3x3 {
    type Output = Matrix2x2;

    fn sub(&self, r: usize, c: usize) -> Self::Output {
        if r > 2 || c > 2 {
            Matrix2x2::repeat(0.)
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

impl MatrixSub for Matrix4x4 {
    type Output = Matrix3x3;

    fn sub(&self, r: usize, c: usize) -> Self::Output {
        if r > 3 || c > 3 {
            Matrix3x3::repeat(0.)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub3x3() -> () {
        let m = Matrix3x3::new(1., 5., 0., -3., 2., 7., 0., 6., -3.);
        let sub = m.sub(0, 2);
        let exp = Matrix2x2::new(-3., 2., 0., 6.);
        assert_eq!(sub, exp);
    }

    #[test]
    fn sub4x4() -> () {
        let m = Matrix4x4::new(-6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.);
        let sub = m.sub(2, 1);
        let exp = Matrix3x3::new(-6., 1., 6., -8., 8., 6., -7., -1., 1.);
        assert_eq!(sub, exp);
    }
}