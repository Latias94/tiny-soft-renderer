use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::time::Duration;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::renderer::Renderer;

fn main() {
    run();
}

fn draw(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);
    let obj_file = "assets/models/african_head.obj";
    let half_width = renderer.width() as f32 / 2.0;
    let half_height = renderer.height() as f32 / 2.0;
    let (models, _materials) = tobj::load_obj(obj_file, &tobj::LoadOptions::default()).unwrap();
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
}

fn redraw(texture: &mut sdl2::render::Texture, renderer: &Renderer) {
    let width = renderer.width() as usize;
    texture
        .update(None, renderer.rgba_bytes(), width * 4)
        .unwrap()
}

fn run() {
    let renderer_width = 800;
    let renderer_height = 800;
    let window_scale = 1;
    let window_width = renderer_width * window_scale;
    let window_height = renderer_height * window_scale;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Playground", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas
        .set_scale(window_scale as f32, window_scale as f32)
        .unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA32, renderer_width, renderer_height)
        .unwrap();

    let mut renderer = Renderer::new(renderer_width, renderer_height, true);

    // draw once temporarily
    draw(&mut renderer);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
            canvas.clear();
            redraw(&mut texture, &renderer);

            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            std::thread::sleep(Duration::new(0, 70_000));
        }
    }
}
