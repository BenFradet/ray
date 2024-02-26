use std::{fs::File, io::{Result, Write}, thread::sleep, time::Duration};

use model::{environment::Environment, projectile::Projectile};

use crate::{base::{colour::Colour, point::Point, vector::Vector}, viewer::{canvas::Canvas, to_file::ToFile}};

mod base;
mod model;
mod viewer;

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).norm() * 11.25,
    };
    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let red = Colour::new(1.0, 0.0, 0.0);

    let canvas_width = 900;
    let canvas_height = 500;

    let f = |canvas: Canvas, position: Point| -> Option<Canvas> {
        canvas.update(position.x as usize, canvas_height - (position.y as usize), red)
    };

    let mut c = Some(Canvas::black(canvas_width, canvas_height))
        .and_then(|canvas| f(canvas, p.position));

    println!("environment: {e}");
    loop {
        println!("projectile: {p}");
        p = tick(e, p);
        if p.position.y <= 0.0 {
            break;
        }
        c = c.and_then(|canvas| f(canvas, p.position));
        sleep(Duration::from_millis(1));
    }

    match c {
        Some(canvas) => {
            let path = "result.ppm";
            match canvas.to_file(path) {
                Ok(()) => println!("successfully written {}", path),
                Err(err) => println!("error writing {}", err),
            }
        },
        None => println!("no canvas"),
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
