use bytemuck::{Pod, Zeroable};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub trait ToFloat32 {
    fn to_f32(&self) -> f32;
}

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
    + ToFloat32
    + Zeroable
    + Pod
{
}

macro_rules! impl_to_float32 {
    ($($t:ty),+)=> {
       $(impl ToFloat32 for $t {
            fn to_f32(&self) -> f32 {
                *self as f32
            }
        })*
    };
}
impl_to_float32!(usize, f32, u32, i32);

macro_rules! impl_number {
    ($($t:ty),+)=> {
       $(impl Number for $t {})*
    };
}

impl_number!(usize, f32, u32, i32);

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
            type Output = $VecType<$Rhs>;

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

impl_vec_type_conversion!(Vec2, f32, u32, x, y);
impl_vec_type_conversion!(Vec3, f32, u32, x, y, z);
impl_vec_type_conversion!(Vec4, f32, u32, x, y, z, w);

impl<T: Number> Vec3<T> {
    pub fn cross(self, rhs: &Self) -> Self {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

macro_rules! impl_vec_to_f32_methods {
    ($VecType:ident, $($field:ident),+) => {
        impl<T: Number + ToFloat32> $VecType<T> {
            pub fn sqrt(&self) -> f32 {
                [$(
                    self.$field.to_f32().powi(2),
                )+]
                .iter().sum::<f32>().sqrt()
            }

            pub fn normalize(&self) -> $VecType<f32> {
                let length = self.sqrt();
                $VecType {
                    $($field: self.$field.to_f32() / length,)+
                }
            }
        }
    };
}

impl_vec_to_f32_methods!(Vec2, x, y);
impl_vec_to_f32_methods!(Vec3, x, y, z);
impl_vec_to_f32_methods!(Vec4, x, y, z, w);

macro_rules! impl_index_and_get_for_vec {
    ($VecType:ident, $T:ty, $($index:expr => $field:ident),+) => {
        impl<T: Number> Index<usize> for $VecType<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    $( $index => &self.$field, )+
                    _ => panic!("Index out of bounds"),
                }
            }
        }

        impl<T: Number> IndexMut<usize> for $VecType<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    $( $index => &mut self.$field, )+
                    _ => panic!("Index out of bounds"),
                }
            }
        }

        impl<T: Number> $VecType<T> {
            #[inline]
            pub fn get(&self, index: usize) -> &T {
                match index {
                    $( $index => &self.$field, )+
                    _ => panic!("Index out of bounds"),
                }
            }
        }
    };
}

impl_index_and_get_for_vec!(Vec2, T, 0 => x, 1 => y);
impl_index_and_get_for_vec!(Vec3, T, 0 => x, 1 => y, 2 => z);
impl_index_and_get_for_vec!(Vec4, T, 0 => x, 1 => y, 2 => z, 3 => w);

// macros to create VecType: vec2, vec3, vec4 and T: usize, f32, u32, i32
macro_rules! impl_vec_common_methods {
    ($VecType:ident, $T:ty, $($field:ident),+) => {
        impl $VecType<$T> {
            #[inline]
            pub const fn new($($field: $T),+) -> Self {
                $VecType {
                    $($field,)+
                }
            }

            #[inline]
            pub const fn splat(value: $T) -> Self {
                $VecType {
                    $($field: value,)+
                }
            }

            pub const ZERO: Self = $VecType {
                $($field: 0 as $T,)+
            };

            pub const ONE: Self = $VecType {
                $($field: 1 as $T,)+
            };

            $(pub const fn $field(&self) -> $T {
                self.$field
            })+
        }
    };
}

impl_vec_common_methods!(Vec2, f32, x, y);
impl_vec_common_methods!(Vec2, u32, x, y);
impl_vec_common_methods!(Vec2, usize, x, y);
impl_vec_common_methods!(Vec2, i32, x, y);
impl_vec_common_methods!(Vec3, f32, x, y, z);
impl_vec_common_methods!(Vec3, u32, x, y, z);
impl_vec_common_methods!(Vec3, usize, x, y, z);
impl_vec_common_methods!(Vec3, i32, x, y, z);
impl_vec_common_methods!(Vec4, f32, x, y, z, w);
impl_vec_common_methods!(Vec4, u32, x, y, z, w);
impl_vec_common_methods!(Vec4, usize, x, y, z, w);
impl_vec_common_methods!(Vec4, i32, x, y, z, w);

macro_rules! impl_vec_common_methods_for_not_f32 {
    ($VecType:ident, $T:ty, $($field:ident),+) => {
        impl $VecType<$T> {
            pub fn dot(&self, rhs: &$VecType<$T>) -> $T {
                let mut result = 0 as $T;
                $(result += self.$field * rhs.$field;)+
                result
            }
        }
    };
}

impl_vec_common_methods_for_not_f32!(Vec2, u32, x, y);
impl_vec_common_methods_for_not_f32!(Vec2, usize, x, y);
impl_vec_common_methods_for_not_f32!(Vec2, i32, x, y);
impl_vec_common_methods_for_not_f32!(Vec3, u32, x, y, z);
impl_vec_common_methods_for_not_f32!(Vec3, usize, x, y, z);
impl_vec_common_methods_for_not_f32!(Vec3, i32, x, y, z);
impl_vec_common_methods_for_not_f32!(Vec4, u32, x, y, z, w);
impl_vec_common_methods_for_not_f32!(Vec4, usize, x, y, z, w);
impl_vec_common_methods_for_not_f32!(Vec4, i32, x, y, z, w);

macro_rules! impl_vec_common_methods_for_f32 {
    ($VecType:ident, $($field:ident),+) => {
        impl $VecType<f32> {
            pub fn dot(&self, rhs: &$VecType<f32>) -> f32 {
                let mut result = 0 as f32;
                $(result += self.$field * rhs.$field;)+
                result
            }
        }
    };
}

impl_vec_common_methods_for_f32!(Vec2, x, y);
impl_vec_common_methods_for_f32!(Vec3, x, y, z);
impl_vec_common_methods_for_f32!(Vec4, x, y, z, w);

pub fn vec2<T: Number>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

pub fn vec3<T: Number>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

pub fn vec4<T: Number>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

macro_rules! impl_vec_from_array {
    ($VecType:ident, $T:ty, $elemCount:expr, $func:ident, $($field:expr),+) => {
        impl From<[$T; $elemCount]> for $VecType<$T> {
            fn from(v: [$T; $elemCount]) -> Self {
                $func($(v[$field],)+)
            }
        }
    };
}
impl_vec_from_array!(Vec2, f32, 2, vec2, 0, 1);
impl_vec_from_array!(Vec2, u32, 2, vec2, 0, 1);
impl_vec_from_array!(Vec2, usize, 2, vec2, 0, 1);
impl_vec_from_array!(Vec2, i32, 2, vec2, 0, 1);
impl_vec_from_array!(Vec3, f32, 3, vec3, 0, 1, 2);
impl_vec_from_array!(Vec3, u32, 3, vec3, 0, 1, 2);
impl_vec_from_array!(Vec3, usize, 3, vec3, 0, 1, 2);
impl_vec_from_array!(Vec3, i32, 3, vec3, 0, 1, 2);
impl_vec_from_array!(Vec4, f32, 4, vec4, 0, 1, 2, 3);
impl_vec_from_array!(Vec4, u32, 4, vec4, 0, 1, 2, 3);
impl_vec_from_array!(Vec4, usize, 4, vec4, 0, 1, 2, 3);
impl_vec_from_array!(Vec4, i32, 4, vec4, 0, 1, 2, 3);

macro_rules! impl_vec_neg_ops {
    ($VecType:ident, $($field:ident),+) => {
        impl<T: Number + Neg<Output = T>> Neg for $VecType<T> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                $VecType {
                    $( $field: -self.$field, )+
                }
            }
        }
    };
}
impl_vec_neg_ops!(Vec2, x, y);
impl_vec_neg_ops!(Vec3, x, y, z);
impl_vec_neg_ops!(Vec4, x, y, z, w);

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use crate::math::vec::{Vec2, Vec3, Vec4};
    use crate::math::{Vec2f, Vec2u, vec3, Vec3i, Vec4u};

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

        let vec = Vec2f { x: 1.1, y: 2.0 };
        let vec_u32: Vec2u = vec.into();
        assert_eq!(vec_u32, Vec2 { x: 1, y: 2 });
    }

    #[test]
    fn test_vec_sqrt() {
        let vec = Vec2 { x: 3, y: 4 };
        assert_eq!(vec.sqrt(), 5.0);
    }

    #[test]
    fn test_vec_normalize() {
        let vec = Vec2 { x: 3, y: 4 };
        let normalized = vec.normalize();
        assert_eq!(normalized, Vec2 { x: 0.6, y: 0.8 });
    }

    #[test]
    fn test_vec_index_and_get() {
        let vec = Vec2 { x: 1, y: 2 };
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);

        let mut vec = Vec2 { x: 1, y: 2 };
        vec[0] = 3;
        vec[1] = 4;
        assert_eq!(vec, Vec2 { x: 3, y: 4 });

        assert_eq!(vec.get(0), &3);
        assert_eq!(vec.get(1), &4);
    }

    #[test]
    fn test_vec_common_methods() {
        let vec = Vec2u::new(1, 2);
        assert_eq!(vec, Vec2u { x: 1, y: 2 });

        let vec = vec3(1, 2, 3);
        assert_eq!(vec, Vec3i { x: 1, y: 2, z: 3 });

        let vec = Vec4u::splat(1);
        assert_eq!(vec, Vec4u { x: 1, y: 1, z: 1, w: 1 });

        let vec1 = Vec4u::new(1, 2, 3, 4);
        let vec2 = Vec4u::new(5, 6, 7, 8);
        assert_eq!(vec1.dot(&vec2), 70);

        let vec1 = Vec2f { x: 1.0, y: 2.0 };
        let vec2 = Vec2f { x: 3.0, y: 4.0 };
        assert_eq!(vec1.dot(&vec2), 11.0);

        assert_eq!(Vec2f::ZERO, Vec2f { x: 0.0, y: 0.0 });
        assert_eq!(Vec2f::ONE, Vec2f { x: 1.0, y: 1.0 });

        assert_eq!(Vec2f::ZERO.x(), 0.0);
        assert_eq!(Vec2f::ONE.y(), 1.0);
        assert_eq!(Vec3i::ZERO.z(), 0);
        assert_eq!(Vec4u::ONE.w(), 1);
    }

    #[test]
    fn test_vec_from_array() {
        let vec = Vec2::from([1, 2]);
        assert_eq!(vec, Vec2 { x: 1, y: 2 });

        let vec = Vec3::from([1, 2, 3]);
        assert_eq!(vec, Vec3 { x: 1, y: 2, z: 3 });

        let vec = Vec4::from([1, 2, 3, 4]);
        assert_eq!(vec, Vec4 { x: 1, y: 2, z: 3, w: 4 });
    }

    #[test]
    fn test_vec_neg() {
        let vec = Vec2 { x: 1, y: 2 };
        assert_eq!(-vec, Vec2 { x: -1, y: -2 });

        let vec = Vec3 { x: 1, y: 2, z: 3 };
        assert_eq!(-vec, Vec3 { x: -1, y: -2, z: -3 });

        let vec = Vec4 { x: 1, y: 2, z: 3, w: 4 };
        assert_eq!(-vec, Vec4 { x: -1, y: -2, z: -3, w: -4 });
    }
}
