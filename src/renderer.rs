use crate::color::Color;
use crate::math::{Vec2f, Vec2u, Vec3, Vec3f};

pub struct Renderer {
    width: u32,
    height: u32,
    flip_y: bool,
    pixels: Vec<Color>,
    y_buffer: Vec<i32>,
}

impl Renderer {
    pub fn new(width: u32, height: u32, flip_y: bool) -> Self {
        Renderer {
            width,
            height,
            flip_y,
            pixels: vec![Color::WHITE; (width * height) as usize],
            y_buffer: vec![-i32::MAX; (width * height) as usize],
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
        self.y_buffer = vec![-i32::MAX; (self.width * self.height) as usize];
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

    pub fn barycentric(t0: &Vec3f, t1: &Vec3f, t2: &Vec3f, p: &Vec3f) -> Vec3<f32> {
        let mut s = [Vec3f::default(); 2];
        for i in 0..2 {
            s[i].x = t2[i] - t0[i];
            s[i].y = t1[i] - t0[i];
            s[i].z = t0[i] - p[i];
        }
        let u = s[0].cross(&s[1]);
        // dont forget that u[2] is integer. If it is zero then triangle ABC is degenerate
        if u.z.abs() > 1e-2 {
            Vec3 {
                x: 1.0 - (u.x + u.y) / u.z,
                y: u.y / u.z,
                z: u.x / u.z,
            }
        } else {
            // in this case generate negative coordinates, it will be thrown away by the rasterizator
            Vec3 {
                x: -1.0,
                y: 1.0,
                z: 1.0,
            }
        }
    }

    pub fn draw_triangle(&mut self, t0: &Vec3f, t1: &Vec3f, t2: &Vec3f, color: Color) {
        let mut bbox_min = Vec2f {
            x: f32::MAX,
            y: f32::MAX,
        };
        let mut bbox_max = Vec2f {
            x: -f32::MAX,
            y: -f32::MAX,
        };
        let clamp = Vec2f {
            x: self.width as f32 - 1.0,
            y: self.height as f32 - 1.0,
        };
        let pts = [t0, t1, t2];
        for pt in pts {
            bbox_min.x = bbox_min.x.min(pt.x).max(0.0);
            bbox_min.y = bbox_min.y.min(pt.y).max(0.0);
            bbox_max.x = bbox_max.x.max(pt.x).min(clamp.x);
            bbox_max.y = bbox_max.y.max(pt.y).min(clamp.y);
        }
        for x in bbox_min.x as u32..=bbox_max.x as u32 {
            for y in bbox_min.y as u32..=bbox_max.y as u32 {
                let p = Vec3f {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0,
                };
                let bc_screen = Renderer::barycentric(t0, t1, t2, &p);

                if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                    continue;
                }
                let index = (y * self.width + x) as usize;
                if self.y_buffer[index] < p.z as i32 {
                    self.y_buffer[index] = p.z as i32;
                    self.draw_pixel(x, y, color);
                }
            }
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use crate::math::{Vec3, Vec3u};

    #[test]
    fn test_barycentric() {
        use crate::math::Vec2u;
        use crate::renderer::Renderer;
        let t0 = Vec3u { x: 0, y: 0, z: 0 };
        let t1 = Vec3u { x: 50, y: 0, z: 0 };
        let t2 = Vec3u { x: 0, y: 50, z: 0 };
        let p = Vec3u { x: 10, y: 10, z: 0 };

        let barycentric_coords =
            Renderer::barycentric(&t0.into(), &t1.into(), &t2.into(), &p.into());
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
