mod common;

use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::renderer::Renderer;

fn main() {
    let title = "Playground";
    let width = 800;
    let height = 800;
    let window_scale = 1;
    let mut renderer = Renderer::new(width, height, true);
    // draw once
    draw(&mut renderer);
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

fn draw_nothing(_renderer: &mut Renderer) {}

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
