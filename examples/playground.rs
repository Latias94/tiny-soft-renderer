mod common;

use crate::common::window_wrapper::WindowWrapper;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::math::Vec2u;
use tiny_soft_renderer::renderer::Renderer;

fn main() {
    let title = "Playground";
    let width = 200;
    let height = 200;
    let window_scale = 4;
    let mut renderer = Renderer::new(width, height, true);
    common::run(title, width, height, window_scale, &mut renderer, draw).unwrap();
}

fn draw(renderer: &mut Renderer, _window: &WindowWrapper) {
    renderer.clear(Color::BLACK);
    let pts = [
        Vec2u { x: 10, y: 10 },
        Vec2u { x: 100, y: 30 },
        Vec2u { x: 190, y: 160 },
    ];
    renderer.draw_triangle(&pts[0], &pts[1], &pts[2], Color::RED);
}
