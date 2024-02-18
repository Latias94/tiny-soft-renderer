use bytemuck::{Pod, Zeroable};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Number:
    Clone
    + Copy
    + Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + SubAssign
    + DivAssign
    + MulAssign
    + Zeroable
    + Pod
{
}

macro_rules! impl_number {
    ($($t:ty),+)=> {
       $(impl Number for $t {})*
    };
}

impl_number!(usize, f32, f64, u32, u64, u128, i32, i64, i128);

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Zeroable)]
pub struct Vec2<T: Number> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Zeroable)]
pub struct Vec3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default, Zeroable)]
pub struct Vec4<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! impl_vec_op {
    ($VecType:ident, $trait:ident, $func:ident, $($field:ident),+) => {
        impl<T: Number> $trait for $VecType<T> {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                $VecType {
                    $($field: self.$field.$func(rhs.$field),)+
                }
            }
        }

        impl<T: Number> $trait<T> for $VecType<T> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self::Output {
                $VecType {
                    $($field: self.$field.$func(rhs),)+
                }
            }
        }
    };
}

impl_vec_op!(Vec2, Add, add, x, y);
impl_vec_op!(Vec2, Sub, sub, x, y);
impl_vec_op!(Vec2, Div, div, x, y);
impl_vec_op!(Vec2, Mul, mul, x, y);

impl_vec_op!(Vec3, Add, add, x, y, z);
impl_vec_op!(Vec3, Sub, sub, x, y, z);
impl_vec_op!(Vec3, Div, div, x, y, z);
impl_vec_op!(Vec3, Mul, mul, x, y, z);

impl_vec_op!(Vec4, Add, add, x, y, z, w);
impl_vec_op!(Vec4, Sub, sub, x, y, z, w);
impl_vec_op!(Vec4, Div, div, x, y, z, w);
impl_vec_op!(Vec4, Mul, mul, x, y, z, w);

macro_rules! impl_vec_op_assign {
    ($VecType:ident, $trait:ident, $func:ident, $($field:ident),+) => {
        impl<T: Number> $trait for $VecType<T> {
            fn $func(&mut self, rhs: Self) {
                $(self.$field.$func(rhs.$field);)+
            }
        }

        impl<T: Number> $trait<T> for $VecType<T> {
            fn $func(&mut self, rhs: T) {
                $(self.$field.$func(rhs);)+
            }
        }
    };
}

impl_vec_op_assign!(Vec2, AddAssign, add_assign, x, y);
impl_vec_op_assign!(Vec2, SubAssign, sub_assign, x, y);
impl_vec_op_assign!(Vec2, DivAssign, div_assign, x, y);
impl_vec_op_assign!(Vec2, MulAssign, mul_assign, x, y);

impl_vec_op_assign!(Vec3, AddAssign, add_assign, x, y, z);
impl_vec_op_assign!(Vec3, SubAssign, sub_assign, x, y, z);
impl_vec_op_assign!(Vec3, DivAssign, div_assign, x, y, z);
impl_vec_op_assign!(Vec3, MulAssign, mul_assign, x, y, z);

impl_vec_op_assign!(Vec4, AddAssign, add_assign, x, y, z, w);
impl_vec_op_assign!(Vec4, SubAssign, sub_assign, x, y, z, w);
impl_vec_op_assign!(Vec4, DivAssign, div_assign, x, y, z, w);
impl_vec_op_assign!(Vec4, MulAssign, mul_assign, x, y, z, w);

macro_rules! impl_vec_vec_op {
    ($VecType:ident, $trait:ident, $func:ident, $T:ty, $Rhs:ty, $OutputT:ty, $($field:ident),+) => {
        impl $trait<$VecType<$Rhs>> for $VecType<$T> {
            type Output = $VecType<$OutputT>;

            fn $func(self, rhs: $VecType<$Rhs>) -> Self::Output {
                $VecType {
                    $($field: (self.$field as $OutputT).$func(rhs.$field as $OutputT),)+
                }
            }
        }
    };
}

impl_vec_vec_op!(Vec2, Add, add, u32, f32, f32, x, y);
impl_vec_vec_op!(Vec3, Add, add, u32, f32, f32, x, y, z);
impl_vec_vec_op!(Vec4, Add, add, u32, f32, f32, x, y, z, w);

impl_vec_vec_op!(Vec2, Add, add, f32, u32, f32, x, y);
impl_vec_vec_op!(Vec3, Add, add, f32, u32, f32, x, y, z);
impl_vec_vec_op!(Vec4, Add, add, f32, u32, f32, x, y, z, w);

macro_rules! impl_vec_num_op {
    ($VecType:ident, $trait:ident, $func:ident, $T:ty, $Rhs:ty, $OutputT:ty, $($field:ident),+) => {
        impl $trait<$Rhs> for $VecType<$T> {
            type Output = $VecType<$OutputT>;

            fn $func(self, rhs: $Rhs) -> Self::Output {
                $VecType {
                    $($field: (self.$field as $OutputT).$func(rhs as $OutputT),)+
                }
            }
        }
    };
}
impl_vec_num_op!(Vec2, Add, add, u32, f32, f32, x, y);
impl_vec_num_op!(Vec3, Add, add, u32, f32, f32, x, y, z);
impl_vec_num_op!(Vec4, Add, add, u32, f32, f32, x, y, z, w);

pub trait ScalarOps<Rhs>: Sized {
    type Output;

    fn scalar_mul(self, rhs: Rhs) -> Self::Output;
}

impl ScalarOps<f32> for u32 {
    type Output = f32;

    fn scalar_mul(self, rhs: f32) -> Self::Output {
        self as f32 * rhs
    }
}

impl ScalarOps<u32> for f32 {
    type Output = f32;

    fn scalar_mul(self, rhs: u32) -> Self::Output {
        self * rhs as f32
    }
}

macro_rules! impl_vec_rhs_scalar_mul {
    ($VecType:ident, $T:ty, $Rhs:ty, $($field:ident),+) => {
        impl Mul<$Rhs> for $VecType<$T>
        where
            $T: ScalarOps<$Rhs> + Number,
            $Rhs: Number,
        {
            type Output = $VecType<<$T as ScalarOps<$Rhs>>::Output>;

            fn mul(self, rhs: $Rhs) -> Self::Output {
                $VecType {
                    $($field: ScalarOps::scalar_mul(self.$field, rhs),)+
                }
            }
        }
    };
}

impl_vec_rhs_scalar_mul!(Vec2, u32, f32, x, y);
impl_vec_rhs_scalar_mul!(Vec3, u32, f32, x, y, z);
impl_vec_rhs_scalar_mul!(Vec4, u32, f32, x, y, z, w);

macro_rules! impl_vec_t_scalar_mul {
    ($VecType:ident, $T:ty, $Rhs:ty, $($field:ident),+) => {
        impl Mul<$Rhs> for $VecType<$T>
        where
            $T: ScalarOps<$Rhs> + Number,
            $Rhs: Number,
        {
            type Output = $VecType<$T>;

            fn mul(self, rhs: $Rhs) -> Self::Output {
                $VecType {
                    $($field: ScalarOps::scalar_mul(self.$field, rhs),)+
                }
            }
        }
    };
}

impl_vec_t_scalar_mul!(Vec2, f32, u32, x, y);
impl_vec_t_scalar_mul!(Vec3, f32, u32, x, y, z);
impl_vec_t_scalar_mul!(Vec4, f32, u32, x, y, z, w);

macro_rules! impl_vec_type_conversion {
    ($VecType:ident, $From:ty, $To:ty, $($field:ident),+) => {
        impl From<$VecType<$From>> for $VecType<$To> {
            fn from(item: $VecType<$From>) -> Self {
                $VecType {
                    $($field: item.$field as $To,)+
                }
            }
        }
    };
}

impl_vec_type_conversion!(Vec2, u32, f32, x, y);
impl_vec_type_conversion!(Vec3, u32, f32, x, y, z);
impl_vec_type_conversion!(Vec4, u32, f32, x, y, z, w);

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::math::vec::{Vec2, Vec3, Vec4};
    use crate::math::{Vec2f, Vec2u};

    #[test]
    fn test_vec2_operations() {
        let vec_a = Vec2 { x: 1.0, y: 2.0 };
        let vec_b = Vec2 { x: 3.0, y: 4.0 };

        let vec_add = vec_a + vec_b;
        assert_eq!(vec_add, Vec2 { x: 4.0, y: 6.0 });

        let vec_sub = vec_a - vec_b;
        assert_eq!(vec_sub, Vec2 { x: -2.0, y: -2.0 });

        let vec_mul = vec_a * vec_b;
        assert_eq!(vec_mul, Vec2 { x: 3.0, y: 8.0 });

        let vec_div = vec_b / Vec2 { x: 1.0, y: 2.0 };
        assert_eq!(vec_div, Vec2 { x: 3.0, y: 2.0 });
    }

    #[test]
    fn test_vec3_chain_operations() {
        let mut vec = Vec3 { x: 1, y: 2, z: 3 };

        vec += Vec3 { x: 1, y: 1, z: 1 };
        vec *= 2;

        assert_eq!(vec, Vec3 { x: 4, y: 6, z: 8 });
    }

    #[test]
    fn test_vec4_assign_operations() {
        let mut vec = Vec4 { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };

        vec -= Vec4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
        assert_eq!(vec, Vec4 { x: 0.0, y: 1.0, z: 2.0, w: 3.0 });

        vec /= Vec4 { x: 1.0, y: 1.0, z: 2.0, w: 3.0 };
        assert_eq!(vec, Vec4 { x: 0.0, y: 1.0, z: 1.0, w: 1.0 });
    }

    #[test]
    fn test_vec2_scalar_mul() {
        let vec = Vec2 { x: 1, y: 2 };
        let scalar = 2.0;

        let result = vec * scalar;
        assert_eq!(result, Vec2 { x: 2.0, y: 4.0 });

        let result = vec * 2;
        assert_eq!(result, Vec2 { x: 2, y: 4 });
    }

    #[test]
    fn test_vec_add() {
        let vec1 = Vec2 { x: 1, y: 2 };
        let vec2 = Vec2 { x: 2.0, y: 3.0 };

        let result = vec1 + vec2;
        assert_eq!(result, Vec2 { x: 3.0, y: 5.0 });
    }

    #[test]
    fn test_vec_cast() {
        let vec = Vec2u { x: 1, y: 2 };
        let vec_f32: Vec2f = vec.into();
        assert_eq!(vec_f32, Vec2 { x: 1.0, y: 2.0 });
    }
}
