use crate::math::{vec2, vec3, Vec2f, Vec3f};
use crate::texture::Texture;
use anyhow::Result;
use bytemuck::Zeroable;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[repr(C)]
#[derive(Clone, Copy, Debug, Zeroable)]
pub struct Vertex {
    pub position: Vec3f,
    pub uv: Vec2f,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.uv == other.uv
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position[0].to_bits().hash(state);
        self.position[1].to_bits().hash(state);
        self.position[2].to_bits().hash(state);
        self.uv[0].to_bits().hash(state);
        self.uv[1].to_bits().hash(state);
    }
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub diffuse: Texture,
}

impl Model {
    #[profiling::function]
    pub fn load_obj_model<P: AsRef<Path> + fmt::Debug>(
        model_path: P,
        diffuse: Texture,
    ) -> Result<Self> {
        let (loaded_models, _materials) = tobj::load_obj(
            model_path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true, // index of uv, normal, and position are the same
                ..Default::default()
            },
        )?;
        let mut data = Model {
            vertices: vec![],
            indices: vec![],
            diffuse,
        };
        let mut unique_vertices = HashMap::new();
        for model in &loaded_models {
            for index in &model.mesh.indices {
                let pos_offset = (3 * index) as usize;
                let tex_coord_offset = (2 * index) as usize;

                let vertex = Vertex {
                    position: vec3(
                        model.mesh.positions[pos_offset],
                        model.mesh.positions[pos_offset + 1],
                        model.mesh.positions[pos_offset + 2],
                    ),
                    // The OBJ format assumes a coordinate system where a vertical coordinate of 0 means the bottom of the image.
                    uv: vec2(
                        model.mesh.texcoords[tex_coord_offset],
                        1.0 - model.mesh.texcoords[tex_coord_offset + 1],
                    ),
                };
                // Vertex deduplication
                if let Some(index) = unique_vertices.get(&vertex) {
                    data.indices.push(*index as u32);
                } else {
                    let index = data.vertices.len();
                    unique_vertices.insert(vertex, index);
                    data.vertices.push(vertex);
                    data.indices.push(index as u32);
                }
            }
        }
        Ok(data)
    }
}
