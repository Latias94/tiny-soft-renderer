use crate::color::Color;
use crate::math::{Vec2, Vec2u};

pub struct Renderer {
    width: u32,
    height: u32,
    flip_y: bool,
    pixels: Vec<Color>,
}

impl Renderer {
    pub fn new(width: u32, height: u32, flip_y: bool) -> Self {
        Renderer {
            width,
            height,
            flip_y,
            pixels: vec![Color::WHITE; (width * height) as usize],
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

    pub fn rgba_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                self.pixels.len() * std::mem::size_of::<Color>(),
            )
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }
        let y = if self.flip_y { self.height - y - 1 } else { y };
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels = vec![color; (self.width * self.height) as usize];
    }

    pub fn draw_line(&mut self, v0: &Vec2u, v1: &Vec2u, color: Color) {
        let mut steep = false;
        let mut x0 = v0.x as i32;
        let mut x1 = v1.x as i32;
        let mut y0 = v0.y as i32;
        let mut y1 = v1.y as i32;
        if (x0 - x1).abs() < (y0 - y1).abs() {
            steep = true;
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }
        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0;
        for x in x0..=x1 {
            if steep {
                self.draw_pixel(y as u32, x as u32, color);
            } else {
                self.draw_pixel(x as u32, y as u32, color);
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }

    pub fn triangle(&mut self, t0: &Vec2u, t1: &Vec2u, t2: &Vec2u, _color: Color) {
        let mut t0: Vec2<f32> = (*t0).into();
        let mut t1: Vec2<f32> = (*t1).into();
        let mut t2: Vec2<f32> = (*t2).into();
        if t0.y > t1.y {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0.y > t2.y {
            std::mem::swap(&mut t0, &mut t2);
        }
        if t1.y > t2.y {
            std::mem::swap(&mut t1, &mut t2);
        }
        let total_height = t2.y - t0.y;
        let t0y = t0.y as i32;
        let t1y = t1.y as i32;
        for y in t0y..=t1y {
            let y = y as f32;
            let segment_height = t1.y - t0.y + 1.0;
            let alpha = (y - t0.y) / total_height;
            let beta = (y - t0.y) / segment_height;
            let a = t0 + (t2 - t0) * alpha;
            let b = t0 + (t1 - t0) * beta;
            self.draw_pixel(a.x as u32, y as u32, Color::RED);
            self.draw_pixel(b.x as u32, y as u32, Color::GREEN);
        }
    }
}
