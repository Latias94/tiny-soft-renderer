use crate::common::window_wrapper::WindowWrapper;
use std::time::Duration;
use tiny_soft_renderer::renderer::Renderer;

pub mod window_wrapper;

pub fn run(
    title: &str,
    width: u32,
    height: u32,
    window_scale: u32,
    renderer: &mut Renderer,
    draw: fn(&mut Renderer),
) -> anyhow::Result<()> {
    let mut window_wrapper = WindowWrapper::new(title, width, height, window_scale);

    loop {
        if window_wrapper.should_quit() {
            break;
        }
        draw(renderer);
        let pixel_row_width = renderer.width() as usize;
        window_wrapper.update(pixel_row_width, renderer.rgba_bytes())?;
        std::thread::sleep(Duration::new(0, 70_000));
    }
    Ok(())
}
