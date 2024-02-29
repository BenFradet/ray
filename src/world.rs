use crate::{
    base::{point::Point, vector::Vector},
    model::{environment::Environment, projectile::Projectile},
};

pub struct World {
    pub p: Projectile,
    pub e: Environment,
}

impl World {
    pub fn new() -> Self {
        let p = Projectile {
            position: Point::new(0.0, 1.0, 0.0),
            velocity: Vector::new(1.0, 1.8, 0.0).norm() * 11.25,
        };
        let e = Environment {
            gravity: Vector::new(0.0, -0.1, 0.0),
            wind: Vector::new(-0.01, 0.0, 0.0),
        };
        Self { p, e }
    }

    pub fn update(&mut self) -> () {
        let position = self.p.position + self.p.velocity;
        let velocity = self.p.velocity + self.e.gravity + self.e.wind;
        self.p = Projectile { position, velocity };
    }
}
