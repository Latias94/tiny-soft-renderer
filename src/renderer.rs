use crate::color::Color;
use crate::math::{Vec2u, Vec3};

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

    pub fn barycentric(t0: &Vec2u, t1: &Vec2u, t2: &Vec2u, p: &Vec2u) -> Vec3<f32> {
        let u = Vec3 {
            x: t2.x as f32 - t0.x as f32,
            y: t1.x as f32 - t0.x as f32,
            z: t0.x as f32 - p.x as f32,
        }
        .cross(&Vec3 {
            x: t2.y as f32 - t0.y as f32,
            y: t1.y as f32 - t0.y as f32,
            z: t0.y as f32 - p.y as f32,
        });

        /* `pts` and `P` has integer value as coordinates
        so `abs(u[2])` < 1 means `u[2]` is 0, that means
        triangle is degenerate, in this case return something with negative coordinates */
        if u.z.abs() < 1.0 {
            return Vec3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            };
        }
        Vec3 {
            x: 1.0 - (u.x + u.y) / u.z,
            y: u.y / u.z,
            z: u.x / u.z,
        }
    }

    pub fn draw_triangle(&mut self, t0: &Vec2u, t1: &Vec2u, t2: &Vec2u, color: Color) {
        let mut bbox_min = Vec2u {
            x: self.width - 1,
            y: self.height - 1,
        };
        let mut bbox_max = Vec2u { x: 0, y: 0 };
        let clamp = Vec2u {
            x: self.width - 1,
            y: self.height - 1,
        };
        let pts = [t0, t1, t2];
        for pt in pts {
            bbox_min.x = bbox_min.x.min(pt.x);
            bbox_min.y = bbox_min.y.min(pt.y);
            bbox_max.x = bbox_max.x.max(pt.x);
            bbox_max.y = bbox_max.y.max(pt.y);
        }
        for x in bbox_min.x..=bbox_max.x {
            for y in bbox_min.y..=bbox_max.y {
                let bc_screen = Renderer::barycentric(t0, t1, t2, &Vec2u { x, y });
                if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                    continue;
                }
                let x = x.min(clamp.x);
                let y = y.min(clamp.y);
                self.draw_pixel(x, y, color);
            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::math::Vec3;

    #[test]
    fn test_barycentric() {
        use crate::math::Vec2u;
        use crate::renderer::Renderer;
        let t0 = Vec2u { x: 0, y: 0 };
        let t1 = Vec2u { x: 50, y: 0 };
        let t2 = Vec2u { x: 0, y: 50 };
        let p = Vec2u { x: 10, y: 10 };

        let barycentric_coords = Renderer::barycentric(&t0, &t1, &t2, &p);
        assert_eq!(
            barycentric_coords,
            Vec3 {
                x: 0.6,
                y: 0.2,
                z: 0.2,
            }
        );
    }
}
