use crate::color::Color;

pub struct Renderer {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
    rgb_pixels: Vec<u8>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Renderer {
            width,
            height,
            pixels: vec![Color::WHITE; (width * height) as usize],
            rgb_pixels: vec![255u8; (width * height * 3) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub fn rgb_pixels(&self) -> &[u8] {
        &self.rgb_pixels
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color;
        self.rgb_pixels[(y * self.width * 3 + x * 3) as usize] = color.r;
        self.rgb_pixels[(y * self.width * 3 + x * 3 + 1) as usize] = color.g;
        self.rgb_pixels[(y * self.width * 3 + x * 3 + 2) as usize] = color.b;
    }

    pub fn clear(&mut self) {
        self.pixels = vec![Color::WHITE; (self.width * self.height) as usize];
        self.rgb_pixels = vec![255u8; (self.width * self.height * 3) as usize];
    }
}
