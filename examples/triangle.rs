mod common;

use crate::common::window_wrapper::WindowWrapper;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::math::{vec3, Vec3i};
use tiny_soft_renderer::renderer::Renderer;

fn main() {
    let title = "Triangle";
    let width = 200;
    let height = 200;
    let window_scale = 4;
    let mut renderer = Renderer::new(width, height, true);
    common::run(title, width, height, window_scale, &mut renderer, draw).unwrap();
}

fn draw(renderer: &mut Renderer, _window: &WindowWrapper) {
    renderer.clear(Color::BLACK);
    // Vec2i t0[3] = {Vec2i(10, 70),   Vec2i(50, 160),  Vec2i(70, 80)};
    // Vec2i t1[3] = {Vec2i(180, 50),  Vec2i(150, 1),   Vec2i(70, 180)};
    // Vec2i t2[3] = {Vec2i(180, 150), Vec2i(120, 160), Vec2i(130, 180)};

    let t0: [Vec3i; 3] = [vec3(10, 70, 0), vec3(50, 160, 0), vec3(70, 80, 0)];
    let t1: [Vec3i; 3] = [vec3(180, 50, 0), vec3(150, 1, 0), vec3(70, 180, 0)];
    let t2: [Vec3i; 3] = [vec3(180, 150, 0), vec3(120, 160, 0), vec3(130, 180, 0)];

    renderer.draw_triangle(
        &t0[0].to_vec_f32(),
        &t0[1].to_vec_f32(),
        &t0[2].to_vec_f32(),
        Color::RED,
    );
    renderer.draw_triangle(
        &t1[0].to_vec_f32(),
        &t1[1].to_vec_f32(),
        &t1[2].to_vec_f32(),
        Color::GREEN,
    );
    renderer.draw_triangle(
        &t2[0].to_vec_f32(),
        &t2[1].to_vec_f32(),
        &t2[2].to_vec_f32(),
        Color::BLUE,
    );
}
