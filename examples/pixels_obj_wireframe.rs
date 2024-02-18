use anyhow::Result;
use pixels::{Error, Pixels, SurfaceTexture};
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::renderer::Renderer;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

fn main() {
    run().unwrap();
}

fn draw_to_frame(renderer: &Renderer, frames: &mut [u8]) {
    let pixels = renderer.rgba_bytes();
    frames.copy_from_slice(pixels);
}

fn draw(renderer: &mut Renderer) -> Result<()> {
    renderer.clear(Color::BLACK);
    let obj_file = "assets/models/african_head.obj";
    let half_width = renderer.width() as f32 / 2.0;
    let half_height = renderer.height() as f32 / 2.0;
    let (models, _materials) = tobj::load_obj(obj_file, &tobj::LoadOptions::default())?;
    for model in &models {
        let mesh = &model.mesh;
        let indices = &mesh.indices;
        let positions = &mesh.positions;

        for f in (0..indices.len()).step_by(3) {
            let [v0, v1, v2] = [
                indices[f] as usize,
                indices[f + 1] as usize,
                indices[f + 2] as usize,
            ];

            let coords = [v0, v1, v2].map(|v| {
                (
                    (positions[3 * v] * half_width + half_width) as i32, // x
                    (positions[3 * v + 1] * half_height + half_height) as i32, // y
                )
            });

            for i in 0..3 {
                let (start, end) = (coords[i], coords[(i + 1) % 3]);
                renderer.draw_line(
                    start.0 as u32,
                    start.1 as u32,
                    end.0 as u32,
                    end.1 as u32,
                    Color::WHITE,
                );
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let renderer_width = 800;
    let renderer_height = 800;
    let window_scale = 1;
    let window_width = renderer_width * window_scale;
    let window_height = renderer_height * window_scale;

    let event_loop = EventLoop::new()?;
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(window_width as f64, window_height as f64);
        winit::window::WindowBuilder::new()
            .with_title("Playground")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_width, window_height, surface_texture)?
    };

    let mut renderer = Renderer::new(renderer_width, renderer_height, true);

    // draw once temporarily
    draw(&mut renderer)?;

    let res = event_loop.run(|event, elwt| {
        // Draw the current frame
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            draw_to_frame(&renderer, pixels.frame_mut());
            if let Err(err) = pixels.render() {
                println!("pixels.render err: {}", err);
                elwt.exit();
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    println!("pixels.resize_surface err: {}", err);
                    elwt.exit();
                    return;
                }
            }

            // draw(&mut renderer);

            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))?;
    Ok(())
}
