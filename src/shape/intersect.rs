use crate::model::ray::Ray;

pub trait Intersect {
    fn intersect(&self, r: Ray) -> Vec<f64>;
}
