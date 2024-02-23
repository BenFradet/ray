use std::fmt::{Display, Formatter, Result};

use crate::base::{point::Point, vector::Vector};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "position: {}, velocity: {}", self.position.to_string(), self.velocity.to_string())
    }
}