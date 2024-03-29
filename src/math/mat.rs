use crate::math::{Vec3, Vec4};
use bytemuck::{Pod, Zeroable};
use std::fmt::Display;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
pub struct Mat4x4 {
    num_rows: usize,
    num_cols: usize,
    m: [f32; 16],
}

impl Mat4x4 {
    pub const ZERO: Mat4x4 = Mat4x4::new([0.0f32; 16]);
    pub const IDENTITY: Mat4x4 = Mat4x4::identity();

    pub const fn new(m: [f32; 16]) -> Self {
        Self {
            num_rows: 4,
            num_cols: 4,
            m,
        }
    }

    pub const fn identity() -> Self {
        Self::new([
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, //
        ])
    }

    pub const fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub const fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        let mut res = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                res[(i, j)] = 0.0;
                for k in 0..4 {
                    res[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        res
    }

    pub fn mul_mat41(&self, rhs: &Mat4x1) -> Mat4x1 {
        let mut res = Mat4x1::ZERO;
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                res[i] += self[(i, j)] * rhs[j];
            }
        }
        res
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        let mut res = Self::identity();
        res[(0, 3)] = x;
        res[(1, 3)] = y;
        res[(2, 3)] = z;
        res.mul(self)
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        let mut res = Self::identity();
        res[(0, 0)] = x;
        res[(1, 1)] = y;
        res[(2, 2)] = z;
        res.mul(self)
    }

    pub fn transpose(&self) -> Self {
        let mut res = Self::identity();
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                res[(i, j)] = self[(j, i)];
            }
        }
        res
    }

    pub const fn at(&self, i: usize, j: usize) -> f32 {
        let index = (i * self.num_cols) + j;
        self.m[index]
    }

    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        self[(i, j)] = value;
    }

    pub fn to_array(&self) -> [f32; 16] {
        self.m
    }

    pub fn as_slice(&self) -> &[f32; 16] {
        &self.m
    }

    pub fn as_slice_mut(&mut self) -> &mut [f32; 16] {
        &mut self.m
    }
}

impl From<[f32; 16]> for Mat4x4 {
    fn from(m: [f32; 16]) -> Self {
        Self::new(m)
    }
}

impl From<[[f32; 4]; 4]> for Mat4x4 {
    fn from(m: [[f32; 4]; 4]) -> Self {
        let mut res = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                res[(i * 4) + j] = m[i][j];
            }
        }
        Self::new(res)
    }
}

impl Index<usize> for Mat4x4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for Mat4x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<(usize, usize)> for Mat4x4 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = (index.0 * self.num_cols) + index.1;
        &self.as_slice()[index]
    }
}

impl IndexMut<(usize, usize)> for Mat4x4 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = (index.0 * self.num_cols) + index.1;
        &mut self.as_slice_mut()[index]
    }
}

impl Deref for Mat4x4 {
    type Target = [f32; 16];

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

impl DerefMut for Mat4x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m
    }
}

impl Display for Mat4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            for j in 0..4 {
                write!(f, "{:.2} ", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Mat4x4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
pub struct Mat4x1 {
    num_rows: usize,
    num_cols: usize,
    m: [f32; 4],
}

impl Mat4x1 {
    pub const ZERO: Mat4x1 = Mat4x1::new([0.0f32; 4]);

    pub const fn new(m: [f32; 4]) -> Self {
        Self {
            num_rows: 4,
            num_cols: 1,
            m,
        }
    }

    pub const fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub const fn num_cols(&self) -> usize {
        self.num_cols
    }
    pub const fn at(&self, i: usize) -> f32 {
        self.m[i]
    }

    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        self[(i, j)] = value;
    }

    pub fn to_array(&self) -> [f32; 4] {
        self.m
    }

    pub fn as_slice(&self) -> &[f32; 4] {
        &self.m
    }

    pub fn as_slice_mut(&mut self) -> &mut [f32; 4] {
        &mut self.m
    }

    pub const fn from_vec3(v: Vec3) -> Self {
        Self::new([v.x, v.y, v.z, 1.0])
    }

    pub const fn from_vec4(v: Vec4) -> Self {
        Self::new([v.x, v.y, v.z, v.w])
    }

    pub fn to_vec3(&self) -> Vec3 {
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = self[3];
        Vec3::new(x / w, y / w, z / w)
    }

    pub fn to_vec4(&self) -> Vec4 {
        let x = self[0];
        let y = self[1];
        let z = self[2];
        let w = self[3];
        Vec4::new(x, y, z, w)
    }
}

impl From<[f32; 4]> for Mat4x1 {
    fn from(m: [f32; 4]) -> Self {
        Self::new(m)
    }
}

impl From<Vec3> for Mat4x1 {
    fn from(v: Vec3) -> Self {
        Self::new([v.x, v.y, v.z, 1.0])
    }
}

impl From<Vec4> for Mat4x1 {
    fn from(v: Vec4) -> Self {
        Self::new([v.x, v.y, v.z, v.w])
    }
}

impl Into<Vec3> for Mat4x1 {
    fn into(self) -> Vec3 {
        Vec3::from([self[0], self[1], self[2]])
    }
}

impl Into<Vec4> for Mat4x1 {
    fn into(self) -> Vec4 {
        Vec4::from([self[0], self[1], self[2], self[3]])
    }
}

impl Index<usize> for Mat4x1 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl IndexMut<usize> for Mat4x1 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

impl Deref for Mat4x1 {
    type Target = [f32; 4];

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

impl DerefMut for Mat4x1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.m
    }
}

impl Display for Mat4x1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(f, "{:.2} ", self.m[i])?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Mat4x1 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = (index.0 * self.num_cols) + index.1;
        &self.as_slice()[index]
    }
}

impl IndexMut<(usize, usize)> for Mat4x1 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = (index.0 * self.num_cols) + index.1;
        &mut self.as_slice_mut()[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mat4f() {
        let m = Mat4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(m.num_rows(), 4);
        assert_eq!(m.num_cols(), 4);

        let m2 = Mat4x4::from([
            [17.0, 18.0, 19.0, 20.0],
            [21.0, 22.0, 23.0, 24.0],
            [25.0, 26.0, 27.0, 28.0],
            [29.0, 30.0, 31.0, 32.0],
        ]);
        let m3 = m.mul(&m2);

        let result = Mat4x4::from([
            [250.0, 260.0, 270.0, 280.0],
            [618.0, 644.0, 670.0, 696.0],
            [986.0, 1028.0, 1070.0, 1112.0],
            [1354.0, 1412.0, 1470.0, 1528.0],
        ]);
        assert_eq!(m3, result);

        let m_identity = Mat4x4::identity();

        let m4 = m_identity.translate(1.0, 2.0, 3.0);
        let result = Mat4x4::from([
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 2.0],
            [0.0, 0.0, 1.0, 3.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(m4, result);

        let m5 = m_identity.scale(3.0, 3.0, 3.0);
        let result = Mat4x4::from([
            [3.0, 0.0, 0.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 3.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(m5, result);

        let m = Mat4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let m6 = m.transpose();
        let result = Mat4x4::from([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);
        assert_eq!(m6, result);

        let m = Mat4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let m7 = m.transpose().transpose();
        assert_eq!(m, m7);

        let mut m = Mat4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        assert_eq!(m[0], 1.0);
        assert_eq!(m[(1, 0)], 5.0);

        m[(0, 0)] = 100.0;
        assert_eq!(
            m,
            Mat4x4::from([
                [100.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ])
        );
    }

    #[test]
    fn test_mat4x1() {
        let m = Mat4x1::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.num_rows(), 4);
        assert_eq!(m.num_cols(), 1);

        assert_eq!(m[0], 1.0);
        assert_eq!(m[1], 2.0);
        assert_eq!(m[2], 3.0);
        assert_eq!(m[3], 4.0);

        let mut m = Mat4x1::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[0], 1.0);
        m[0] = 100.0;
        assert_eq!(m[0], 100.0);

        let mut m = Mat4x1::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[0], 1.0);
        m[(1, 0)] = 100.0;
        assert_eq!(m[1], 100.0);

        let v = Vec4::from([1.0, 2.0, 3.0, 4.0]);
        let m = Mat4x1::from(v);
        assert_eq!(m, Mat4x1::from([1.0, 2.0, 3.0, 4.0]));

        let m44 = Mat4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let result = m44.mul_mat41(&m);
        assert_eq!(result, Mat4x1::from([30.0, 70.0, 110.0, 150.0]));

        let m = Mat4x1::from([1.0, 2.0, 3.0, 4.0]);
        let v = Vec4::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.to_vec4(), v);

        let m = Mat4x1::from([1.0, 2.0, 3.0, 4.0]);
        let v = Vec3::from([1.0, 2.0, 3.0]);
        assert_eq!(m.to_vec3(), v);

        let v = Vec4::from([1.0, 2.0, 3.0, 4.0]);
        let m = Mat4x1::from(v);
        assert_eq!(m, Mat4x1::from([1.0, 2.0, 3.0, 4.0]));

        let v = Vec3::from([1.0, 2.0, 3.0]);
        let m = Mat4x1::from(v);
        assert_eq!(m, Mat4x1::from([1.0, 2.0, 3.0, 1.0]));
    }
}
