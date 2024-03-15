use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use math::{colour::Colour, matrix::Matrix4x4, point::Point, vector::Vector};
use model::{
    camera::Camera, intersection::{Intersection}, material::Material, point_light::PointLight, ray::Ray, sphere::Sphere, world::World
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
    let width = 500;
    let height = 250;
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

    let wall_mat = Material::default()
        .colour(Colour::new(1., 0.9, 0.9))
        .specular(0.);
    let wall_t = Matrix4x4::scaling(10., 0.01, 10.);
    let floor = Sphere::new(wall_t)
        .unwrap()
        .material(wall_mat);
    let left_wall = Sphere::new(wall_t.rotate_x(FRAC_PI_2).rotate_y(-FRAC_PI_4).translate(0., 0., 5.))
        .unwrap()
        .material(wall_mat);
    let right_wall = Sphere::new(wall_t.rotate_x(FRAC_PI_2).rotate_y(FRAC_PI_4).translate(0., 0., 5.))
        .unwrap()
        .material(wall_mat);

    let middle_mat = Material::new(Colour::new(0.1, 1., 0.5), 0.1, 0.7, 0.3);
    let middle = Sphere::new(Matrix4x4::translation(-0.5, 1., 0.5))
        .unwrap()
        .material(middle_mat);

    let right_mat = Material::new(Colour::new(0.5, 1., 0.1), 0.1, 0.7, 0.3);
    let right = Sphere::new(Matrix4x4::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .unwrap()
        .material(right_mat);

    let left_mat = Material::new(Colour::new(1., 0.8, 0.1), 0.1, 0.7, 0.3);
    let left = Sphere::new(Matrix4x4::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
        .unwrap()
        .material(left_mat);

    let world = World::new(
        vec![floor, left_wall, right_wall, left, middle, right],
        vec![PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE)],
    );

    let camera = Camera::new(width_usize, height_usize, FRAC_PI_3)
        .transform(Matrix4x4::view_transform(
            Point::new(0., 1.5, -5.),
            Point::new(0., 1., 0.),
            Vector::new(0., 1., 0.)),
        )
        .unwrap();

    let mut canvas = Canvas::black(camera.hsize, camera.vsize);
    canvas.render(&camera, &world);

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
                let path = "result_world.ppm";
                match canvas.to_file(path) {
                    Ok(()) => println!("successfully written {}", path),
                    Err(err) => println!("error writing {}", err),
                }

                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        window.request_redraw();
    });
}

#[allow(dead_code)]
fn lightning_colour(i: Intersection, ray: Ray, light: PointLight) -> Colour {
    let point = ray.position(i.t);
    let normal = i.object.normal_at(point);
    let eye = -ray.direction;
    i.object.material.lightning(light, point, eye, normal)
}
