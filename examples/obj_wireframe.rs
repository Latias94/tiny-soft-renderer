mod common;

use crate::common::window_wrapper::WindowWrapper;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::math::Vec2u;
use tiny_soft_renderer::renderer::Renderer;
use tobj::Model;

fn main() {
    let title = "Playground";
    let width = 800;
    let height = 800;
    let window_scale = 1;
    let mut renderer = Renderer::new(width, height, true);
    let obj_file = "assets/models/african_head.obj";
    let (models, _materials) = tobj::load_obj(obj_file, &tobj::LoadOptions::default()).unwrap();
    let model = &models[0];
    // draw once
    draw(model, &mut renderer);
    common::run(
        title,
        width,
        height,
        window_scale,
        &mut renderer,
        draw_nothing,
    )
    .unwrap();
}

fn draw_nothing(_renderer: &mut Renderer, _window: &WindowWrapper) {}

fn draw(model: &Model, renderer: &mut Renderer) {
    renderer.clear(Color::BLACK);
    let half_width = renderer.width() as f32 / 2.0;
    let half_height = renderer.height() as f32 / 2.0;
    let mesh = &model.mesh;
    let indices = &mesh.indices;
    let positions = &mesh.positions;

    for f in (0..indices.len()).step_by(3) {
        let [v0, v1, v2] = [
            indices[f] as usize,
            indices[f + 1] as usize,
            indices[f + 2] as usize,
        ];

        let coords = [v0, v1, v2].map(|v| Vec2u {
            x: ((positions[3 * v] + 1.0) * half_width) as u32,
            y: ((positions[3 * v + 1] + 1.0) * half_height) as u32,
        });

        for i in 0..3 {
            let (start, end) = (coords[i], coords[(i + 1) % 3]);
            renderer.draw_line(&start, &end, Color::WHITE);
        }
    }
}
