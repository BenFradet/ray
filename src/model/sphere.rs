use crate::math::{
    matrix::Matrix4x4, matrix_const::MatrixConst, matrix_invert::MatrixInvert,
    matrix_transpose::MatrixTranspose, point::Point, vector::Vector,
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub t: Matrix4x4,
    inv_t: Matrix4x4,
    t_inv_t: Matrix4x4,
}

impl Sphere {
    pub fn new(t: Matrix4x4) -> Option<Self> {
        let inv = t.invert();
        inv.map(|inv_t| Sphere {
            center: Point::new(0., 0., 0.),
            radius: 1.,
            t,
            inv_t,
            t_inv_t: inv_t.transpose(),
        })
    }

    pub fn id() -> Self {
        Sphere {
            center: Point::new(0., 0., 0.),
            radius: 1.,
            t: Matrix4x4::ID,
            inv_t: Matrix4x4::ID,
            t_inv_t: Matrix4x4::ID,
        }
    }

    pub fn t(mut self, t: Matrix4x4) -> Option<Self> {
        self.t = t;
        t.invert().map(|inv_t| {
            self.inv_t = inv_t;
            self.t_inv_t = inv_t.transpose();
            self
        })
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = self.inv_t * world_point;
        let object_normal = object_point - self.center;
        let world_normal = self.t_inv_t * object_normal;
        world_normal.w(0.0).norm()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::math::round::Round;

    use super::*;

    #[test]
    fn normal_at_scale_rz_sphere() -> () {
        let m = Matrix4x4::rotation_z(PI / 5.).scale(1., 0.5, 1.);
        let sphere = Sphere::new(m);
        assert!(sphere.is_some());
        let s = sphere.unwrap();
        let s2 = SQRT_2 / 2.;
        let res = s.normal_at(Point::new(0., s2, -s2));
        assert_eq!(res.rounded(5), vec![0., 0.97014, -0.24254, 0.]);
    }

    #[test]
    fn normal_at_translated_sphere() -> () {
        let sphere = Sphere::new(Matrix4x4::translation(0., 1., 0.));
        assert!(sphere.is_some());
        let s = sphere.unwrap();
        let s2 = SQRT_2 / 2.;
        let res = s.normal_at(Point::new(0., 1. + s2, -s2));
        assert_eq!(res.rounded(5), Vector::new(0., s2, -s2).rounded(5));
    }

    #[test]
    fn normal_at() -> () {
        let s = Sphere::id();
        assert_eq!(s.normal_at(Point::new(1., 0., 0.)), Vector::new(1., 0., 0.));
        assert_eq!(s.normal_at(Point::new(0., 1., 0.)), Vector::new(0., 1., 0.));
        assert_eq!(s.normal_at(Point::new(0., 0., 1.)), Vector::new(0., 0., 1.));
        let s3 = 3f64.sqrt() / 3.;
        let res = s.normal_at(Point::new(s3, s3, s3));
        assert_eq!(res, Vector::new(s3, s3, s3));
        assert_eq!(res.norm(), res);
    }

    #[test]
    fn t() -> () {
        let s = Sphere::id();
        assert_eq!(s.t, Matrix4x4::ID);
        let t = Matrix4x4::translation(2., 3., 4.);
        let new_s = s.t(t);
        assert!(new_s.is_some());
        assert_eq!(new_s.unwrap().t, t);
    }

    #[test]
    fn new() -> () {
        let t = Matrix4x4::translation(2., 3., 4.);
        let inv_t = t.invert().unwrap();
        let s = Sphere::new(t).unwrap();
        assert_eq!(s.t, t);
        assert_eq!(s.inv_t, inv_t);
    }
}
