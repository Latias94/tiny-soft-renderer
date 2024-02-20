use std::ops::{Mul, MulAssign};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 0xff }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn random() -> Self {
        let r = (rand::random::<f32>() * 255.0) as u8;
        let g = (rand::random::<f32>() * 255.0) as u8;
        let b = (rand::random::<f32>() * 255.0) as u8;
        Color::rgb(r, g, b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::rgba(
            (self.r as f32 * rhs) as u8,
            (self.g as f32 * rhs) as u8,
            (self.b as f32 * rhs) as u8,
            self.a,
        )
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}
