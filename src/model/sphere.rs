use crate::math::{matrix::Matrix4x4, matrix_const::MatrixConst, point::Point};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub t: Matrix4x4,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            center: Point::new(0., 0., 0.),
            radius: 1.,
            t: Matrix4x4::ID,
        }
    }

    pub fn t(mut self, t: Matrix4x4) -> Self {
        self.t = t;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() -> () {
        let s = Sphere::new();
        assert_eq!(s.t, Matrix4x4::ID);
        let t = Matrix4x4::translation(2., 3., 4.);
        let new_s = s.t(t);
        assert_eq!(new_s.t, t);
    }

    #[test]
    fn new() -> () {
        let s = Sphere::new();
        assert_eq!(s.center, Point::new(0., 0., 0.));
        assert_eq!(s.radius, 1.);
    }
}