use std::{thread::sleep, time::Duration};

use model::{environment::Environment, projectile::Projectile};

use crate::math::{point::Point, vector::Vector};

mod math;
mod model;

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).norm(),
    };
    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    println!("environment: {e}");
    loop {
        println!("projectile: {p}");
        p = tick(e, p);
        sleep(Duration::from_secs(1));
    }
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile {
        position,
        velocity,
    }
}
