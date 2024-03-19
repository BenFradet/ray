use crate::math::{point::Point, vector::Vector};

pub trait Normal {
    fn normal_at(&self, object_point: Point) -> Vector;
}
