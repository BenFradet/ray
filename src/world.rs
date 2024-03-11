use std::f64::consts::FRAC_PI_6;
use std::fmt::{Display, Formatter, Result};

use rand::Rng;

use crate::math::{matrix::Matrix4x4, point::Point, vector::Vector};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "gravity: {}, wind: {}",
            self.gravity.to_string(),
            self.wind.to_string()
        )
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "position: {}, velocity: {}",
            self.position.to_string(),
            self.velocity.to_string()
        )
    }
}

#[allow(dead_code)]
pub struct World {
    pub p: Projectile,
    pub e: Environment,
}

impl World {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn update(&mut self) -> () {
        let position = self.p.position + self.p.velocity;
        let velocity = self.p.velocity + self.e.gravity + self.e.wind;
        self.p = Projectile { position, velocity };
    }
}

#[allow(dead_code)]
pub struct Clock {
    pub display: Point,
    reference: Point,
    t: Matrix4x4,
    s: Matrix4x4,
}

impl Clock {
    #[allow(dead_code)]
    pub fn new(w: usize, h: usize) -> Self {
        let scale = 3. / 8.;
        let wf = w as f64;
        let hf = h as f64;
        Self {
            display: Point::new(0., 1., 0.),
            reference: Point::new(0., 1., 0.),
            t: Matrix4x4::translation(wf / 2., hf / 2., 0.),
            s: Matrix4x4::scaling(scale * wf, scale * hf, 1.),
        }
    }

    #[allow(dead_code)]
    fn rand_angle(divisions: f64, jitter: f64) -> f64 {
        let mut rand = rand::thread_rng();
        let r: f64 = rand.gen();
        let j = r * jitter * 2. - jitter;
        let f = (r * divisions).trunc();
        f * FRAC_PI_6 + j
    }

    #[allow(dead_code)]
    pub fn update(&mut self) -> () {
        let angle = Self::rand_angle(12., 0.4);
        let new = self.t * self.s * Matrix4x4::rotation_z(angle) * self.reference;
        self.display = new;
    }
}
