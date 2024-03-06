use super::{matrix::Matrix2x2, matrix_indexing::MatrixIndexing, matrix_size::MatrixSize, matrix_sub::MatrixSub};

pub trait Cofactor {
    fn minor(&self, r: usize, c: usize) -> f64;
    fn cofactor(&self, r: usize, c: usize) -> f64;
}

impl<A: MatrixSub<Output = B>, B: Det> Cofactor for A {
    fn minor(&self, r: usize, c: usize) -> f64 {
        let sub = self.sub(r, c);
        sub.det()
    }

    fn cofactor(&self, r: usize, c: usize) -> f64 {
        let minor = self.minor(r, c);
        if r + c % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

pub trait Det {
    fn det(&self) -> f64;
}

impl <A: Cofactor + MatrixSize + MatrixIndexing> Det for A {
    fn det(&self) -> f64 {
        let mut det = 0.0;
        for i in 0..Self::SIZE {
            det = det + self.at(0, i) * self.cofactor(0, i);
        }
        det
        
    }
}

impl Det for Matrix2x2 {
    fn det(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[1][0] * self.m[0][1]
    }
}

#[cfg(test)]
mod tests {
    use crate::math::matrix::{Matrix3x3, Matrix4x4};

    use super::*;

    #[test]
    fn det4x4() -> () {
        let m = Matrix4x4::new(-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.det(), -4071.0);
    }

    #[test]
    fn det3x3() -> () {
        let m = Matrix3x3::new(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.det(), -196.0);
    }

    #[test]
    fn det2x2() -> () {
        let m = Matrix2x2::new(1.0, 5.0, -3.0, 2.0);
        assert_eq!(m.det(), 17.0);
    }

    #[test]
    fn cofactor() -> () {
        let m = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn minor() -> () {
        let m = Matrix3x3::new(3.0, 5.0, 0.0, 2.0, -1.0, 7.0, 6.0, -1.0, 5.0);
        assert_eq!(m.minor(1, 0), 25.0)
    }
}