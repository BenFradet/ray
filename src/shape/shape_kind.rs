use crate::{math::{point::Point, vector::Vector}, model::ray::Ray};

use super::{intersect::Intersect, normal::Normal, plane::Plane, sphere::Sphere};

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ShapeKind {
    P(Plane),
    S(Sphere),
}

impl Intersect for ShapeKind {
    fn intersect(&self, r: &Ray) -> Vec<f64> {
        match self {
            ShapeKind::P(plane) => plane.intersect(r),
            ShapeKind::S(sphere) => sphere.intersect(r),
        }
    }
}

impl Normal for ShapeKind {
    fn normal_at(&self, object_point: Point) -> Vector {
        match self {
            ShapeKind::P(plane) => plane.normal_at(object_point),
            ShapeKind::S(sphere) => sphere.normal_at(object_point),
        }
    }
}