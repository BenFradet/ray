use std::{f64::consts::FRAC_PI_3, rc::Rc};

use pixels::{Error, Pixels, SurfaceTexture};
use ray::{
    math::{colour::Colour, matrix::Matrix4x4, point::Point, vector::Vector},
    model::{camera::Camera, material::Material, point_light::PointLight, world::World},
    patterns::pattern::Pattern,
    shapes::shape::Shape,
    viewer::{canvas::Canvas, drawable::Drawable, to_file::ToFile},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

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

    let checker_pattern = Pattern::new_checker(
        Colour::WHITE,
        Colour::BLACK,
        Matrix4x4::scaling(0.1, 0.1, 0.1),
    )
    .unwrap();
    let floor_and_roof = Shape::new_cube(Matrix4x4::scaling(20., 10., 20.))
        .unwrap()
        .material(Material::default().pattern(checker_pattern));

    let stripe_pattern = Pattern::new_stripe(
        Colour::hex("4f", "3b", "3b").unwrap_or(Colour::WHITE),
        Colour::hex("3b", "4f", "3b").unwrap_or(Colour::BLACK),
        Matrix4x4::scaling(0.1, 0.1, 0.1),
    ).unwrap();

    let walls = Shape::new_cube(Matrix4x4::scaling(18., 15., 18.))
        .unwrap()
        .material(Material::default().pattern(stripe_pattern));

    let world = World::new(
        vec![Rc::new(floor_and_roof), Rc::new(walls)],
        vec![PointLight::new(Point::new(-8., 8., -8.), Colour::WHITE)],
    );

    let vt = {
        let to = Point::new(0., 1., 0.);
        let up = Vector::new(0., 1., 0.);
        move |eye: Point| -> Matrix4x4 { Matrix4x4::view_transform(eye, to, up) }
    };

    let mut eye = Point::new(0., 1.5, -7.);
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
