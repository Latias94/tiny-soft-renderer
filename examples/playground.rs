mod common;

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

fn draw(renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);
    let t0 = [
        Vec2u { x: 10, y: 70 },
        Vec2u { x: 50, y: 160 },
        Vec2u { x: 70, y: 80 },
    ];
    let t1 = [
        Vec2u { x: 180, y: 50 },
        Vec2u { x: 150, y: 1 },
        Vec2u { x: 70, y: 180 },
    ];
    let t2 = [
        Vec2u { x: 180, y: 150 },
        Vec2u { x: 120, y: 160 },
        Vec2u { x: 130, y: 180 },
    ];
    renderer.triangle(t0[0], t0[1], t0[2], Color::RED);
    renderer.triangle(t1[0], t1[1], t1[2], Color::WHITE);
    renderer.triangle(t2[0], t2[1], t2[2], Color::GREEN);
}
