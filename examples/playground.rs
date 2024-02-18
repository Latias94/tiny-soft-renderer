mod common;

use crate::common::window_wrapper::WindowWrapper;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::math::Vec2u;
use tiny_soft_renderer::renderer::Renderer;

fn main() {
    let title = "Playground";
    let width = 800;
    let height = 800;
    let window_scale = 1;
    let mut renderer = Renderer::new(width, height, true);
    common::run(title, width, height, window_scale, &mut renderer, draw).unwrap();
}

fn draw(renderer: &mut Renderer, _window: &WindowWrapper) {
    renderer.clear(Color::BLACK);
    renderer.rasterize(
        &Vec2u { x: 20, y: 34 },
        &Vec2u { x: 744, y: 400 },
        Color::RED,
    );
    renderer.rasterize(
        &Vec2u { x: 120, y: 434 },
        &Vec2u { x: 444, y: 400 },
        Color::GREEN,
    );
    renderer.rasterize(
        &Vec2u { x: 330, y: 463 },
        &Vec2u { x: 594, y: 200 },
        Color::BLUE,
    );
}
