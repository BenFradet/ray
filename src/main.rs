use math::colour::Colour;
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
use crate::{viewer::to_file::ToFile, world::World};

mod math;
mod model;
mod viewer;
mod world;

fn main() -> Result<(), Error> {
    let width = 900;
    let height = 500;
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

    let mut world = World::new();
    let mut canvas = Canvas::black(width as usize, height as usize);
    canvas.update(
        world.p.position.x as usize,
        height_usize - (world.p.position.y as usize),
        Colour::RED,
    );

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
                let path = "result.ppm";
                match canvas.to_file(path) {
                    Ok(()) => println!("successfully written {}", path),
                    Err(err) => println!("error writing {}", err),
                }

                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        world.update();
        if world.p.position.y > 0.0 {
            canvas.update(
                world.p.position.x as usize,
                height_usize - (world.p.position.y as usize),
                Colour::RED,
            );
        }
        window.request_redraw();
    });
}
