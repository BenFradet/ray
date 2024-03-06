use super::det::Det;

pub trait MatrixInvertible {
    fn is_invertible(&self) -> bool;
}

impl <T: Det> MatrixInvertible for T {
    fn is_invertible(&self) -> bool {
        self.det() != 0.0
    }
}

#[cfg(test)]
mod tests {
    use crate::math::matrix::Matrix4x4;

    use super::*;

    #[test]
    fn is_invertible() -> () {
        let m = Matrix4x4::new(6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -8.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0);
        assert!(m.is_invertible())
    }

    #[test]
    fn is_not_invertible() -> () {
        let m = Matrix4x4::new(-4.0, 2.0, -2.0, 3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0);
        assert!(!m.is_invertible())
    }
}