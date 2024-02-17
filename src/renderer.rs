use crate::color::Color;

pub struct Renderer {
    width: u32,
    height: u32,
    flip_y: bool,
    pixels: Vec<Color>,
    rgb_pixels: Vec<u8>,
}

impl Renderer {
    pub fn new(width: u32, height: u32, flip_y: bool) -> Self {
        Renderer {
            width,
            height,
            flip_y,
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
        if x >= self.width || y >= self.height {
            return;
        }
        let y = if self.flip_y { self.height - y - 1 } else { y };
        self.pixels[(y * self.width + x) as usize] = color;
        self.rgb_pixels[(y * self.width * 3 + x * 3) as usize] = color.r;
        self.rgb_pixels[(y * self.width * 3 + x * 3 + 1) as usize] = color.g;
        self.rgb_pixels[(y * self.width * 3 + x * 3 + 2) as usize] = color.b;
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels = vec![color; (self.width * self.height) as usize];
        let mut index = 0;
        for _y in 0..self.height {
            for _x in 0..self.width {
                self.rgb_pixels[index] = color.r;
                self.rgb_pixels[index + 1] = color.g;
                self.rgb_pixels[index + 2] = color.b;
                index += 3;
            }
        }
    }

    pub fn draw_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
        let mut steep = false;
        let mut x0 = x0 as i32;
        let mut x1 = x1 as i32;
        let mut y0 = y0 as i32;
        let mut y1 = y1 as i32;
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
                self.set_pixel(y as u32, x as u32, color);
            } else {
                self.set_pixel(x as u32, y as u32, color);
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }
}
