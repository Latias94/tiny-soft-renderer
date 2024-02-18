use crate::color::Color;
use crate::math::Vec2f;
use anyhow::Result;
use image::io::Reader as ImageReader;
use std::path::Path;

pub struct Texture {
    pub pixels: Vec<Color>,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn get_color(&self, uv: &Vec2f) -> Color {
        let x = (uv.x * self.width as f32) as u32;
        let y = (uv.y * self.height as f32) as u32;
        let index = (y * self.width + x) as usize;
        self.pixels[index]
    }
}

pub fn load_tga_texture<P: AsRef<Path>>(path: P) -> Result<Texture> {
    let path = path.as_ref();
    let img = ImageReader::open(path)?.decode()?;
    let img = img.into_rgba8();
    let width = img.width();
    let height = img.height();
    let pixels = img
        .pixels()
        .map(|p| Color {
            r: p[0],
            g: p[1],
            b: p[2],
            a: p[3],
        })
        .collect();
    Ok(Texture {
        pixels,
        width,
        height,
    })
}
