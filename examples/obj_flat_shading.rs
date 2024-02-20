mod common;

use sdl2::keyboard::Scancode;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::math::{vec3, Vec2u, Vec3};
use tiny_soft_renderer::model::Model;
use tiny_soft_renderer::renderer::Renderer;
use tiny_soft_renderer::texture::Texture;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const WINDOW_SCALE: u32 = 1;

enum DrawMode {
    Diffuse,
    Flat,
    RandomColor,
    Wireframe,
}

fn main() {
    let title = "Playground, press A/S/D to change shading mode";
    let mut renderer = Renderer::new(WIDTH, HEIGHT, true);
    let diffuse = Texture::load_tga_texture("assets/textures/african_head_diffuse.tga").unwrap();
    let model = Model::load_obj_model("assets/models/african_head.obj", diffuse).unwrap();

    common::run(
        title,
        WIDTH,
        HEIGHT,
        WINDOW_SCALE,
        &mut renderer,
        |renderer, window| {
            let mut draw_mode = DrawMode::Diffuse;
            if window.is_key_pressed(Scancode::A) {
                draw_mode = DrawMode::Flat;
            } else if window.is_key_pressed(Scancode::S) {
                draw_mode = DrawMode::RandomColor;
            } else if window.is_key_pressed(Scancode::D) {
                draw_mode = DrawMode::Wireframe;
            }
            draw(&model, renderer, draw_mode);
        },
    )
    .unwrap();
}

fn draw(model: &Model, renderer: &mut Renderer, draw_mode: DrawMode) {
    renderer.clear(Color::BLACK);
    let half_width = renderer.width() as f32 / 2.0;
    let half_height = renderer.height() as f32 / 2.0;
    let light_dir = vec3(0.0, 0.0, -1.0);

    for index in model.indices.chunks(3) {
        let [v0, v1, v2] = [
            model.vertices[index[0] as usize].position,
            model.vertices[index[1] as usize].position,
            model.vertices[index[2] as usize].position,
        ];

        // points of the triangle
        let world_coords = [v0, v1, v2];

        let screen_coords = world_coords.map(|v| Vec3 {
            x: (v.x + 1.0) * half_width,
            y: (v.y + 1.0) * half_height,
            z: v.z,
        });

        match draw_mode {
            DrawMode::Diffuse => {
                let normal = (world_coords[2] - world_coords[0])
                    .cross(&(world_coords[1] - world_coords[0]))
                    .normalize();
                let intensity = normal.dot(&light_dir);
                if intensity > 0.0 {
                    let uvs = [
                        model.vertices[index[0] as usize].uv,
                        model.vertices[index[1] as usize].uv,
                        model.vertices[index[2] as usize].uv,
                    ];

                    renderer.draw_triangle_uv(
                        &screen_coords[0],
                        &screen_coords[1],
                        &screen_coords[2],
                        &uvs[0],
                        &uvs[1],
                        &uvs[2],
                        &model.diffuse,
                        intensity,
                    );
                }
            }
            DrawMode::Flat => {
                let normal = (world_coords[2] - world_coords[0])
                    .cross(&(world_coords[1] - world_coords[0]))
                    .normalize();
                let intensity = normal.dot(&light_dir);
                // Back-face culling
                if intensity > 0.0 {
                    let color = Color::rgb(
                        (intensity * 255.0) as u8,
                        (intensity * 255.0) as u8,
                        (intensity * 255.0) as u8,
                    );
                    renderer.draw_triangle(
                        &screen_coords[0],
                        &screen_coords[1],
                        &screen_coords[2],
                        color,
                    );
                }
            }
            DrawMode::RandomColor => {
                renderer.draw_triangle(
                    &screen_coords[0],
                    &screen_coords[1],
                    &screen_coords[2],
                    Color::random(),
                );
            }
            DrawMode::Wireframe => {
                let screen_coords_2d = screen_coords.map(|v| Vec2u {
                    x: v.x as u32,
                    y: v.y as u32,
                });
                renderer.draw_line(&screen_coords_2d[0], &screen_coords_2d[1], Color::WHITE);
                renderer.draw_line(&screen_coords_2d[1], &screen_coords_2d[2], Color::WHITE);
                renderer.draw_line(&screen_coords_2d[2], &screen_coords_2d[0], Color::WHITE);
            }
        }
    }
}
