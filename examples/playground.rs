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
    renderer.draw_line(13, 20, 80, 40, Color::WHITE);
    renderer.draw_line(20, 13, 40, 80, Color::RED);
    renderer.draw_line(80, 40, 13, 20, Color::RED);
}

fn redraw(texture: &mut sdl2::render::Texture, renderer: &Renderer) {
    let width = renderer.width() as usize;
    // 3 = rgb, we use `PixelFormatEnum::RGB24` in `run()`
    texture
        .update(None, renderer.rgb_pixels(), width * 3)
        .unwrap()
}

fn run() {
    let renderer_width = 100;
    let renderer_height = 100;
    let window_scale = 8;
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
        .create_texture_target(PixelFormatEnum::RGB24, renderer_width, renderer_height)
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
