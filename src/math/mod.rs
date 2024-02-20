pub mod mat;
pub mod vec;

pub use mat::*;
pub use vec::*;

pub type Vec2u = TVec2<u32>;
pub type Vec2i = TVec2<i32>;
pub type Vec2f = TVec2<f32>;
pub type Vec2 = Vec2f;

pub type Vec3u = TVec3<u32>;
pub type Vec3i = TVec3<i32>;
pub type Vec3f = TVec3<f32>;
pub type Vec3 = Vec3f;

pub type Vec4u = TVec4<u32>;
pub type Vec4i = TVec4<i32>;
pub type Vec4f = TVec4<f32>;
pub type Vec4 = Vec4f;

pub type Mat4 = Mat4x4;
pub type Mat44 = Mat4x4;
pub type Mat41 = Mat4x1;
