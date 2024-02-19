use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::Sdl;
use std::time::{Duration, Instant};

pub struct WindowWrapper {
    _sdl_context: Sdl,
    canvas: WindowCanvas,
    event_pump: sdl2::EventPump,
    texture: Texture,
    title: String,
    last_frame: Instant,
    frame_count: u32,
    fps: u32,
}

#[profiling::all_functions]
impl WindowWrapper {
    pub fn new(title: &str, width: u32, height: u32, window_scale: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, width * window_scale, height * window_scale)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            // .present_vsync()
            .build()
            .unwrap();
        canvas
            .set_scale(window_scale as f32, window_scale as f32)
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_target(PixelFormatEnum::RGBA32, width, height)
            .unwrap();

        Self {
            _sdl_context: sdl_context,
            canvas,
            event_pump,
            texture,
            title: title.to_string(),
            last_frame: Instant::now(),
            frame_count: 0,
            fps: 0,
        }
    }

    pub fn update(&mut self, pixel_row_width: usize, pixels: &[u8]) -> Result<()> {
        profiling::finish_frame!();
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);

        self.frame_count += 1;
        if delta > Duration::from_secs(1) {
            self.fps = self.frame_count;
            self.frame_count = 0;
            self.last_frame = now;
            let window = self.canvas.window_mut();
            window.set_title(&format!("{} | FPS: {}", self.title, self.fps))?;
        }

        self.texture.update(None, pixels, pixel_row_width * 4)?;
        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).unwrap();
        self.canvas.present();
        Ok(())
    }

    pub fn should_quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_key_pressed(&self, code: Scancode) -> bool {
        let keyboard_state = sdl2::keyboard::KeyboardState::new(&self.event_pump);
        keyboard_state.is_scancode_pressed(code)
    }
}
