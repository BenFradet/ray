use crate::{
    math::{point::Point, vector::Vector},
    model::ray::Ray,
};

use super::{cube::Cube, intersect::Intersect, normal::Normal, plane::Plane, sphere::Sphere};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ShapeKind {
    P(Plane),
    S(Sphere),
    C(Cube),
}

impl Intersect for ShapeKind {
    fn intersect(&self, r: &Ray) -> Vec<f64> {
        match self {
            ShapeKind::P(plane) => plane.intersect(r),
            ShapeKind::S(sphere) => sphere.intersect(r),
            ShapeKind::C(cube) => cube.intersect(r),
        }
    }
}

impl Normal for ShapeKind {
    fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            ShapeKind::P(plane) => plane.normal_at(object_point),
            ShapeKind::S(sphere) => sphere.normal_at(object_point),
            ShapeKind::C(cube) => cube.normal_at(object_point),
        }
    }
}
