mod common;

use sdl2::keyboard::Scancode;
use tiny_soft_renderer::color::Color;
use tiny_soft_renderer::math::{Vec2f, Vec2u, Vec3f};
use tiny_soft_renderer::renderer::Renderer;
use tiny_soft_renderer::texture;
use tiny_soft_renderer::texture::Texture;
use tobj::Model;

enum DrawMode {
    Diffuse,
    Flat,
    RandomColor,
    Wireframe,
}

fn main() {
    let title = "Playground, press A/S/D to change shading mode";
    let width = 800;
    let height = 800;
    let window_scale = 1;
    let mut renderer = Renderer::new(width, height, true);
    let obj_file = "assets/models/african_head.obj";
    let (models, _materials) = tobj::load_obj(obj_file, &tobj::LoadOptions::default()).unwrap();
    let model = &models[0];
    let diffuse = texture::load_tga_texture("assets/textures/african_head_diffuse.tga").unwrap();
    common::run(
        title,
        width,
        height,
        window_scale,
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
            draw(model, renderer, &diffuse, draw_mode);
        },
    )
    .unwrap();
}

fn draw(model: &Model, renderer: &mut Renderer, diffuse: &Texture, draw_mode: DrawMode) {
    renderer.clear(Color::BLACK);
    let half_width = renderer.width() as f32 / 2.0;
    let half_height = renderer.height() as f32 / 2.0;

    let mesh = &model.mesh;
    let indices = &mesh.indices;
    let positions = &mesh.positions;

    let light_dir = Vec3f {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };

    for f in (0..indices.len()).step_by(3) {
        let [v0, v1, v2] = [
            indices[f] as usize,
            indices[f + 1] as usize,
            indices[f + 2] as usize,
        ];

        // points of the triangle
        let world_coords = [v0, v1, v2].map(|v| Vec3f {
            x: positions[3 * v],
            y: positions[3 * v + 1],
            z: positions[3 * v + 2],
        });

        let screen_coords = world_coords.map(|v| Vec3f {
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
                        Vec2f {
                            x: mesh.texcoords[2 * v0],
                            y: mesh.texcoords[2 * v0 + 1],
                        },
                        Vec2f {
                            x: mesh.texcoords[2 * v1],
                            y: mesh.texcoords[2 * v1 + 1],
                        },
                        Vec2f {
                            x: mesh.texcoords[2 * v2],
                            y: mesh.texcoords[2 * v2 + 1],
                        },
                    ];

                    renderer.draw_triangle_uv(
                        &screen_coords[0],
                        &screen_coords[1],
                        &screen_coords[2],
                        &uvs[0],
                        &uvs[1],
                        &uvs[2],
                        diffuse,
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
