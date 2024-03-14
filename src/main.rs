use std::f64::consts::FRAC_PI_4;

use math::{colour::Colour, matrix::Matrix4x4, point::Point};
use model::{
    intersection::{Intersection, IntersectionHit},
    material::Material,
    point_light::PointLight,
    ray::Ray,
    sphere::Sphere,
};
use pixels::{Error, Pixels, SurfaceTexture};
use viewer::canvas::Canvas;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::viewer::drawable::Drawable;
use crate::viewer::to_file::ToFile;

mod math;
mod model;
mod viewer;

fn main() -> Result<(), Error> {
    let width = 1000;
    let height = 500;
    let width_usize = width as usize;
    let height_usize = height as usize;

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("ray")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture)?
    };

    let ray_origin = Point::new(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / width as f64;
    let half_wall_size = wall_size / 2.;
    let material = Material::new().colour(Colour::new(1., 0.5, 0.92));
    let sphere1 = Sphere::new(Matrix4x4::scaling(0.5, 1., 1.).rotate_z(FRAC_PI_4))
        .unwrap()
        .material(material);
    let sphere2 = Sphere::new(Matrix4x4::scaling(0.5, 1., 1.).rotate_z(-FRAC_PI_4))
        .unwrap()
        .material(material);
    let point_light = PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE);

    let mut canvas = Canvas::black(width_usize, height_usize);
    canvas.update(ray_origin.x as usize, ray_origin.y as usize, Colour::RED);

    let mut x = 0;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            canvas.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                println!("pixels.render {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                let path = "result_circle_3d.ppm";
                match canvas.to_file(path) {
                    Ok(()) => println!("successfully written {}", path),
                    Err(err) => println!("error writing {}", err),
                }

                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if x < width {
            x = x + 1;

            for y in 0..height {
                let world_x = -half_wall_size + pixel_size * x as f64;
                let world_y = half_wall_size - pixel_size * y as f64;

                let position = Point::new(world_x, world_y, wall_z);
                let ray = Ray::new(ray_origin, (position - ray_origin).norm());
                let is1 = ray.intersections(&sphere1);
                let is2 = ray.intersections(&sphere2);

                if let Some(hit1) = is1.hit() {
                    let colour = lightning_colour(hit1, ray, point_light);
                    canvas.update(x as usize, y as usize, colour);
                }

                if let Some(hit2) = is2.hit() {
                    let colour = lightning_colour(hit2, ray, point_light);
                    canvas.update(x as usize, y as usize, colour);
                }
            }
            window.request_redraw();
        }
    });
}

fn lightning_colour(i: Intersection, ray: Ray, light: PointLight) -> Colour {
    let point = ray.position(i.t);
    let normal = i.object.normal_at(point);
    let eye = -ray.direction;
    i.object.material.lightning(light, point, eye, normal)
}
