use crate::{
    math::{
        matrix::Matrix4x4, matrix_const::MatrixConst, matrix_invert::MatrixInvert,
        matrix_transpose::MatrixTranspose, point::Point, vector::Vector,
    },
    model::{intersection::Intersection, material::Material, ray::Ray},
};

use super::{
    intersect::Intersect, normal::Normal, plane::Plane, shape_kind::ShapeKind, sphere::Sphere,
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Shape {
    t: Matrix4x4,
    pub inv_t: Matrix4x4,
    t_inv_t: Matrix4x4,
    pub material: Material,
    underlying: ShapeKind,
}

impl Shape {
    pub fn new(s: ShapeKind, t: Matrix4x4) -> Option<Self> {
        let inv = t.invert();
        inv.map(|inv_t| Self {
            t,
            inv_t,
            t_inv_t: inv_t.transpose(),
            material: Material::default(),
            underlying: s,
        })
    }

    pub fn new_sphere(t: Matrix4x4) -> Option<Self> {
        let inv = t.invert();
        inv.map(|inv_t| Self {
            t,
            inv_t,
            t_inv_t: inv_t.transpose(),
            material: Material::default(),
            underlying: ShapeKind::S(Sphere {}),
        })
    }

    pub fn new_plane(t: Matrix4x4) -> Option<Self> {
        let inv = t.invert();
        inv.map(|inv_t| Self {
            t,
            inv_t,
            t_inv_t: inv_t.transpose(),
            material: Material::default(),
            underlying: ShapeKind::P(Plane {}),
        })
    }

    pub fn id_sphere() -> Self {
        Self {
            t: Matrix4x4::ID,
            inv_t: Matrix4x4::ID,
            t_inv_t: Matrix4x4::ID,
            material: Material::default(),
            underlying: ShapeKind::S(Sphere {}),
        }
    }

    pub fn id(s: ShapeKind) -> Self {
        Self {
            t: Matrix4x4::ID,
            inv_t: Matrix4x4::ID,
            t_inv_t: Matrix4x4::ID,
            material: Material::default(),
            underlying: s,
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

    pub fn material(mut self, m: Material) -> Self {
        self.material = m;
        self
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = self.inv_t * world_point;
        let object_normal = self.underlying.normal_at(object_point);
        let world_normal = self.t_inv_t * object_normal;
        world_normal.w(0.0).norm()
    }

    pub fn intersections(&self, r: &Ray) -> Vec<Intersection> {
        let t_ray = r.transform(self.inv_t);
        let ts = self.underlying.intersect(&t_ray);
        ts.iter().map(|t| Intersection::new(*t, *self)).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::math::round::Round;

    use super::*;

    #[test]
    fn normal_at_transformed_shape() -> () {
        let t = Matrix4x4::rotation_z(PI / 5.).scale(1., 0.5, 1.);
        let s = Shape::new_sphere(t).unwrap_or(Shape::id_sphere());
        let s2 = SQRT_2 / 2.;
        let res = s.normal_at(Point::new(0., s2, -s2));
        assert_eq!(res.rounded(5), vec![0., 0.97014, -0.24254, 0.]);
    }

    #[test]
    fn normal_at_translated_shape() -> () {
        let s = Shape::new_sphere(Matrix4x4::translation(0., 1., 0.)).unwrap_or(Shape::id_sphere());
        let res = s.normal_at(Point::new(0., 1.70711, -0.70711));
        assert_eq!(res.rounded(5), vec![0., 0.70711, -0.70711, 0.]);
    }

    #[test]
    fn intersections_translated_shape() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::translation(5., 0., 0.)).unwrap_or(Shape::id_sphere());
        let res = s.intersections(&r);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn intersections_scaled_shape() -> () {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Shape::new_sphere(Matrix4x4::scaling(2., 2., 2.)).unwrap_or(Shape::id_sphere());
        let res = s.intersections(&r);
        assert_eq!(res[0].t, 3.);
        assert_eq!(res[1].t, 7.);
    }

    #[test]
    fn material() -> () {
        let s = Shape::id_sphere();
        assert_eq!(s.material, Material::default());
        let m = Material::default().ambient(1.);
        let new_s = s.material(m);
        assert_eq!(new_s.material, m);
    }

    #[test]
    fn normal_at_scale_rz_sphere() -> () {
        let m = Matrix4x4::rotation_z(PI / 5.).scale(1., 0.5, 1.);
        let sphere = Shape::new_sphere(m);
        assert!(sphere.is_some());
        let s = sphere.unwrap();
        let s2 = SQRT_2 / 2.;
        let res = s.normal_at(Point::new(0., s2, -s2));
        assert_eq!(res.rounded(5), vec![0., 0.97014, -0.24254, 0.]);
    }

    #[test]
    fn normal_at_translated_sphere() -> () {
        let sphere = Shape::new_sphere(Matrix4x4::translation(0., 1., 0.));
        assert!(sphere.is_some());
        let s = sphere.unwrap();
        let s2 = SQRT_2 / 2.;
        let res = s.normal_at(Point::new(0., 1. + s2, -s2));
        assert_eq!(res.rounded(5), Vector::new(0., s2, -s2).rounded(5));
    }

    #[test]
    fn normal_at() -> () {
        let s = Shape::id_sphere();
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
        let s = Shape::id_sphere();
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
        let s = Shape::new_sphere(t).unwrap();
        assert_eq!(s.t, t);
        assert_eq!(s.inv_t, inv_t);
    }
}
