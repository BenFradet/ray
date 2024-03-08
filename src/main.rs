use math::{colour::Colour, point::Point};
use model::{intersection::IntersectionHit, ray::Ray, sphere::Sphere};
use pixels::{Error, Pixels, SurfaceTexture};
use viewer::canvas::Canvas;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
use world::Clock;

use crate::viewer::drawable::Drawable;
use crate::viewer::to_file::ToFile;

mod math;
mod model;
mod viewer;
mod world;

fn main() -> Result<(), Error> {
    let width = 500;
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
    let sphere = Sphere::new();

    let mut canvas = Canvas::black(width_usize, height_usize);
    canvas.update(
        ray_origin.x as usize,
        ray_origin.y as usize,
        Colour::RED,
    );

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
                let path = "result_circle.ppm";
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
                let r = Ray::new(ray_origin, (position - ray_origin).norm());
                let is = r.intersections(sphere);

                if is.hit().is_some() {
                    canvas.update(
                        x as usize,
                        y as usize,
                        Colour::RED,
                    );
                    window.request_redraw();
                }
            }
        }
    });
}
