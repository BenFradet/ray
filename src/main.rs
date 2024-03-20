use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

use math::{colour::Colour, matrix::Matrix4x4, point::Point, vector::Vector};
use model::{camera::Camera, material::Material, point_light::PointLight, world::World};
use pattern::pattern::Pattern;
use pixels::{Error, Pixels, SurfaceTexture};
use shape::shape::Shape;
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
mod pattern;
mod shape;
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

    let stripe_pattern = Pattern::new_stripe(
        Colour::WHITE,
        Colour::BLACK,
        Matrix4x4::rotation_z(FRAC_PI_2),
    )
    .unwrap_or(Pattern::id_stripe(Colour::WHITE, Colour::BLACK));
    let gradient_pattern = Pattern::id_gradient(Colour::WHITE, Colour::BLACK);

    let wall_mat = Material::default()
        .colour(Colour::new(1., 0.9, 0.9))
        .specular(0.);
    let wall_t = Matrix4x4::scaling(10., 0.01, 10.);
    let floor = Shape::new_plane(wall_t).unwrap().material(wall_mat);
    let left_wall = Shape::new_plane(
        wall_t
            .rotate_x(FRAC_PI_2)
            .rotate_y(-FRAC_PI_4)
            .translate(0., 0., 5.),
    )
    .unwrap()
    .material(wall_mat.pattern(gradient_pattern));
    let right_wall = Shape::new_plane(
        wall_t
            .rotate_x(FRAC_PI_2)
            .rotate_y(FRAC_PI_4)
            .translate(0., 0., 5.),
    )
    .unwrap()
    .material(wall_mat);

    let middle_mat = Material::new(Colour::new(0.1, 1., 0.5), 0.1, 0.7, 0.3).pattern(stripe_pattern);
    let middle = Shape::new_sphere(Matrix4x4::translation(-0.5, 1., 0.5))
        .unwrap()
        .material(middle_mat);

    let right_mat = Material::new(Colour::new(0.5, 1., 0.1), 0.1, 0.7, 0.3);
    let right = Shape::new_sphere(Matrix4x4::scaling(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5))
        .unwrap()
        .material(right_mat);

    let left_mat = Material::new(Colour::new(1., 0.8, 0.1), 0.1, 0.7, 0.3);
    let left = Shape::new_sphere(Matrix4x4::scaling(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75))
        .unwrap()
        .material(left_mat);

    let world = World::new(
        vec![floor, left_wall, right_wall, left, middle, right],
        vec![PointLight::new(Point::new(-10., 10., -10.), Colour::WHITE)],
    );

    let to = Point::new(0., 1., 0.);
    let up = Vector::new(0., 1., 0.);
    let vt = move |eye: Point| -> Matrix4x4 { Matrix4x4::view_transform(eye, to, up) };

    let mut eye = Point::new(0., 1.5, -5.);
    let mut camera = Camera::new(width_usize, height_usize, FRAC_PI_3)
        .transform(vt(eye))
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
            } else if input.key_pressed(VirtualKeyCode::W) {
                eye = Point::new(eye.x, eye.y + 0.5, eye.z);
            } else if input.key_pressed(VirtualKeyCode::S) {
                eye = Point::new(eye.x, eye.y - 0.5, eye.z);
            } else if input.key_pressed(VirtualKeyCode::A) {
                eye = Point::new(eye.x - 0.5, eye.y, eye.z);
            } else if input.key_pressed(VirtualKeyCode::D) {
                eye = Point::new(eye.x + 0.5, eye.y, eye.z);
            } else if input.key_pressed(VirtualKeyCode::Q) {
                eye = Point::new(eye.x, eye.y, eye.z - 0.5);
            } else if input.key_pressed(VirtualKeyCode::E) {
                eye = Point::new(eye.x, eye.y, eye.z + 0.5);
            }
            camera = camera.transform(vt(eye)).unwrap();
            canvas.render(&camera, &world);
            window.request_redraw();
        }
    });
}

//fn lightning_colour(i: Intersection, ray: Ray, light: PointLight) -> Colour {
//    let point = ray.position(i.t);
//    let normal = i.object.normal_at(point);
//    let eye = -ray.direction;
//    i.object.material.lightning(light, point, eye, normal, false)
//}
