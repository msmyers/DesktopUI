#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Matrix3x2 {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl Matrix3x2 {}
impl ::std::default::Default for Matrix3x2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Matrix3x2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Matrix3x2").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("M31", &self.M31).field("M32", &self.M32).finish()
    }
}
impl ::std::cmp::PartialEq for Matrix3x2 {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.M31 == other.M31 && self.M32 == other.M32
    }
}
impl ::std::cmp::Eq for Matrix3x2 {}
unsafe impl ::windows::runtime::Abi for Matrix3x2 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Matrix3x2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Matrix3x2;f4;f4;f4;f4;f4;f4)");
}
impl Matrix3x2 {
    pub fn identity() -> Self {
        Self { M11: 1.0, M12: 0.0, M21: 0.0, M22: 1.0, M31: 0.0, M32: 0.0 }
    }
    pub fn translation(x: f32, y: f32) -> Self {
        Self { M11: 1.0, M12: 0.0, M21: 0.0, M22: 1.0, M31: x, M32: y }
    }
    pub fn rotation(angle: f32, x: f32, y: f32) -> Self {
        #[repr(C)]
        pub struct D2D_POINT_2F {
            pub x: f32,
            pub y: f32,
        }
        #[link(name = "d2d1")]
        extern "system" {
            fn D2D1MakeRotateMatrix(angle: f32, center: D2D_POINT_2F, matrix: *mut Matrix3x2);
        }
        let mut matrix = Self::default();
        unsafe {
            D2D1MakeRotateMatrix(angle, D2D_POINT_2F { x, y }, &mut matrix);
        }
        matrix
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 + rhs.M11,
            M12: self.M12 + rhs.M12,
            M21: self.M21 + rhs.M21,
            M22: self.M22 + rhs.M22,
            M31: self.M31 + rhs.M31,
            M32: self.M32 + rhs.M32,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 - rhs.M11,
            M12: self.M12 - rhs.M12,
            M21: self.M21 - rhs.M21,
            M22: self.M22 - rhs.M22,
            M31: self.M31 - rhs.M31,
            M32: self.M32 - rhs.M32,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 * rhs.M11 + self.M12 * rhs.M21,
            M12: self.M11 * rhs.M12 + self.M12 * rhs.M22,
            M21: self.M21 * rhs.M11 + self.M22 * rhs.M21,
            M22: self.M21 * rhs.M12 + self.M22 * rhs.M22,
            M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + rhs.M31,
            M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + rhs.M32,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            M11: self.M11 * rhs,
            M12: self.M12 * rhs,
            M21: self.M21 * rhs,
            M22: self.M22 * rhs,
            M31: self.M31 * rhs,
            M32: self.M32 * rhs,
        }
    }
}
impl ::std::ops::Add<Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Add<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Sub<Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Sub<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Mul<Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<f32> for Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: f32) -> Matrix3x2 {
        self.impl_mul_f32(rhs)
    }
}
impl ::std::ops::Mul<f32> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: f32) -> Matrix3x2 {
        self.impl_mul_f32(rhs)
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Matrix4x4 {
    pub M11: f32,
    pub M12: f32,
    pub M13: f32,
    pub M14: f32,
    pub M21: f32,
    pub M22: f32,
    pub M23: f32,
    pub M24: f32,
    pub M31: f32,
    pub M32: f32,
    pub M33: f32,
    pub M34: f32,
    pub M41: f32,
    pub M42: f32,
    pub M43: f32,
    pub M44: f32,
}
impl Matrix4x4 {}
impl ::std::default::Default for Matrix4x4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Matrix4x4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Matrix4x4")
            .field("M11", &self.M11)
            .field("M12", &self.M12)
            .field("M13", &self.M13)
            .field("M14", &self.M14)
            .field("M21", &self.M21)
            .field("M22", &self.M22)
            .field("M23", &self.M23)
            .field("M24", &self.M24)
            .field("M31", &self.M31)
            .field("M32", &self.M32)
            .field("M33", &self.M33)
            .field("M34", &self.M34)
            .field("M41", &self.M41)
            .field("M42", &self.M42)
            .field("M43", &self.M43)
            .field("M44", &self.M44)
            .finish()
    }
}
impl ::std::cmp::PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M13 == other.M13 && self.M14 == other.M14 && self.M21 == other.M21 && self.M22 == other.M22 && self.M23 == other.M23 && self.M24 == other.M24 && self.M31 == other.M31 && self.M32 == other.M32 && self.M33 == other.M33 && self.M34 == other.M34 && self.M41 == other.M41 && self.M42 == other.M42 && self.M43 == other.M43 && self.M44 == other.M44
    }
}
impl ::std::cmp::Eq for Matrix4x4 {}
unsafe impl ::windows::runtime::Abi for Matrix4x4 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Matrix4x4 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4)");
}
impl Matrix4x4 {
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 + rhs.M11,
            M12: self.M12 + rhs.M12,
            M13: self.M13 + rhs.M13,
            M14: self.M14 + rhs.M14,
            M21: self.M21 + rhs.M21,
            M22: self.M22 + rhs.M22,
            M23: self.M23 + rhs.M23,
            M24: self.M24 + rhs.M24,
            M31: self.M31 + rhs.M31,
            M32: self.M32 + rhs.M32,
            M33: self.M33 + rhs.M33,
            M34: self.M34 + rhs.M34,
            M41: self.M41 + rhs.M41,
            M42: self.M42 + rhs.M42,
            M43: self.M43 + rhs.M43,
            M44: self.M44 + rhs.M44,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 - rhs.M11,
            M12: self.M12 - rhs.M12,
            M13: self.M13 - rhs.M13,
            M14: self.M14 - rhs.M14,
            M21: self.M21 - rhs.M21,
            M22: self.M22 - rhs.M22,
            M23: self.M23 - rhs.M23,
            M24: self.M24 - rhs.M24,
            M31: self.M31 - rhs.M31,
            M32: self.M32 - rhs.M32,
            M33: self.M33 - rhs.M33,
            M34: self.M34 - rhs.M34,
            M41: self.M41 - rhs.M41,
            M42: self.M42 - rhs.M42,
            M43: self.M43 - rhs.M43,
            M44: self.M44 - rhs.M44,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 * rhs.M11 + self.M12 * rhs.M21 + self.M13 * rhs.M31 + self.M14 * rhs.M41,
            M12: self.M11 * rhs.M12 + self.M12 * rhs.M22 + self.M13 * rhs.M32 + self.M14 * rhs.M42,
            M13: self.M11 * rhs.M13 + self.M12 * rhs.M23 + self.M13 * rhs.M33 + self.M14 * rhs.M43,
            M14: self.M11 * rhs.M14 + self.M12 * rhs.M24 + self.M13 * rhs.M34 + self.M14 * rhs.M44,
            M21: self.M21 * rhs.M11 + self.M22 * rhs.M21 + self.M23 * rhs.M31 + self.M24 * rhs.M41,
            M22: self.M21 * rhs.M12 + self.M22 * rhs.M22 + self.M23 * rhs.M32 + self.M24 * rhs.M42,
            M23: self.M21 * rhs.M13 + self.M22 * rhs.M23 + self.M23 * rhs.M33 + self.M24 * rhs.M43,
            M24: self.M21 * rhs.M14 + self.M22 * rhs.M24 + self.M23 * rhs.M34 + self.M24 * rhs.M44,
            M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + self.M33 * rhs.M31 + self.M34 * rhs.M41,
            M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + self.M33 * rhs.M32 + self.M34 * rhs.M42,
            M33: self.M31 * rhs.M13 + self.M32 * rhs.M23 + self.M33 * rhs.M33 + self.M34 * rhs.M43,
            M34: self.M31 * rhs.M14 + self.M32 * rhs.M24 + self.M33 * rhs.M34 + self.M34 * rhs.M44,
            M41: self.M41 * rhs.M11 + self.M42 * rhs.M21 + self.M43 * rhs.M31 + self.M44 * rhs.M41,
            M42: self.M41 * rhs.M12 + self.M42 * rhs.M22 + self.M43 * rhs.M32 + self.M44 * rhs.M42,
            M43: self.M41 * rhs.M13 + self.M42 * rhs.M23 + self.M43 * rhs.M33 + self.M44 * rhs.M43,
            M44: self.M41 * rhs.M14 + self.M42 * rhs.M24 + self.M43 * rhs.M34 + self.M44 * rhs.M44,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            M11: self.M11 * rhs,
            M12: self.M12 * rhs,
            M13: self.M13 * rhs,
            M14: self.M14 * rhs,
            M21: self.M21 * rhs,
            M22: self.M22 * rhs,
            M23: self.M23 * rhs,
            M24: self.M24 * rhs,
            M31: self.M31 * rhs,
            M32: self.M32 * rhs,
            M33: self.M33 * rhs,
            M34: self.M34 * rhs,
            M41: self.M41 * rhs,
            M42: self.M42 * rhs,
            M43: self.M43 * rhs,
            M44: self.M44 * rhs,
        }
    }
}
impl ::std::ops::Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Add<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Sub<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Sub<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        self.impl_mul_f32(rhs)
    }
}
impl ::std::ops::Mul<f32> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        self.impl_mul_f32(rhs)
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Plane {
    pub Normal: Vector3,
    pub D: f32,
}
impl Plane {}
impl ::std::default::Default for Plane {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Plane {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Plane").field("Normal", &self.Normal).field("D", &self.D).finish()
    }
}
impl ::std::cmp::PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.Normal == other.Normal && self.D == other.D
    }
}
impl ::std::cmp::Eq for Plane {}
unsafe impl ::windows::runtime::Abi for Plane {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Plane {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Quaternion {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl Quaternion {}
impl ::std::default::Default for Quaternion {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Quaternion {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Quaternion").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
impl ::std::cmp::PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.W == other.W
    }
}
impl ::std::cmp::Eq for Quaternion {}
unsafe impl ::windows::runtime::Abi for Quaternion {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Quaternion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Rational {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl Rational {}
impl ::std::default::Default for Rational {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Rational {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Rational").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::std::cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::std::cmp::Eq for Rational {}
unsafe impl ::windows::runtime::Abi for Rational {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Rational {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Rational;u4;u4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Vector2 {
    pub X: f32,
    pub Y: f32,
}
impl Vector2 {}
impl ::std::default::Default for Vector2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Vector2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Vector2").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::std::cmp::PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::std::cmp::Eq for Vector2 {}
unsafe impl ::windows::runtime::Abi for Vector2 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Vector2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Vector2;f4;f4)");
}
impl Vector2 {
    pub fn new(X: f32, Y: f32) -> Self {
        Self { X, Y }
    }
    pub fn zero() -> Self {
        Self { X: 0f32, Y: 0f32 }
    }
    pub fn one() -> Self {
        Self { X: 1f32, Y: 1f32 }
    }
    pub fn unit_x() -> Self {
        Self { X: 1.0, Y: 0.0 }
    }
    pub fn unit_y() -> Self {
        Self { X: 0.0, Y: 1.0 }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.X * rhs.X + self.Y * rhs.Y
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self { X: self.X + rhs.X, Y: self.Y + rhs.Y }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self { X: self.X - rhs.X, Y: self.Y - rhs.Y }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self { X: self.X / rhs.X, Y: self.Y / rhs.Y }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self { X: self.X / rhs, Y: self.Y / rhs }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self { X: self.X * rhs.X, Y: self.Y * rhs.Y }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self { X: self.X * rhs, Y: self.Y * rhs }
    }
}
impl ::std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Add<Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Sub<Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Div<Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        self.impl_div(&rhs)
    }
}
impl ::std::ops::Div<&Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        self.impl_div(rhs)
    }
}
impl ::std::ops::Div<Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        self.impl_div(&rhs)
    }
}
impl ::std::ops::Div<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        self.impl_div(rhs)
    }
}
impl ::std::ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        self.impl_div_f32(rhs)
    }
}
impl ::std::ops::Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        self.impl_div_f32(rhs)
    }
}
impl ::std::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        self.impl_mul_f32(rhs)
    }
}
impl ::std::ops::Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        self.impl_mul_f32(rhs)
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Vector3 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl Vector3 {}
impl ::std::default::Default for Vector3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Vector3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Vector3").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::std::cmp::PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::std::cmp::Eq for Vector3 {}
unsafe impl ::windows::runtime::Abi for Vector3 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Vector3 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4)");
}
impl Vector3 {
    pub fn new(X: f32, Y: f32, Z: f32) -> Self {
        Self { X, Y, Z }
    }
    pub fn zero() -> Self {
        Self { X: 0f32, Y: 0f32, Z: 0f32 }
    }
    pub fn one() -> Self {
        Self { X: 1f32, Y: 1f32, Z: 1f32 }
    }
    pub fn unit_x() -> Self {
        Self { X: 1.0, Y: 0.0, Z: 0.0 }
    }
    pub fn unit_y() -> Self {
        Self { X: 0.0, Y: 1.0, Z: 0.0 }
    }
    pub fn unit_z() -> Self {
        Self { X: 0.0, Y: 0.0, Z: 1.0 }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.X * rhs.X + self.Y * rhs.Y + self.Z * rhs.Z
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self { X: self.X + rhs.X, Y: self.Y + rhs.Y, Z: self.Z + rhs.Z }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self { X: self.X - rhs.X, Y: self.Y - rhs.Y, Z: self.Z - rhs.Z }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self { X: self.X / rhs.X, Y: self.Y / rhs.Y, Z: self.Z / rhs.Z }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self { X: self.X / rhs, Y: self.Y / rhs, Z: self.Z / rhs }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self { X: self.X * rhs.X, Y: self.Y * rhs.Y, Z: self.Z * rhs.Z }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self { X: self.X * rhs, Y: self.Y * rhs, Z: self.Z * rhs }
    }
}
impl ::std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Vector3 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Vector3 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Vector3 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Vector3 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Vector3 {
        self.impl_div(&rhs)
    }
}
impl ::std::ops::Div<&Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: &Vector3) -> Vector3 {
        self.impl_div(rhs)
    }
}
impl ::std::ops::Div<Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Vector3 {
        self.impl_div(&rhs)
    }
}
impl ::std::ops::Div<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: &Vector3) -> Vector3 {
        self.impl_div(rhs)
    }
}
impl ::std::ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        self.impl_div_f32(rhs)
    }
}
impl ::std::ops::Div<f32> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        self.impl_div_f32(rhs)
    }
}
impl ::std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Vector3) -> Vector3 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Vector3) -> Vector3 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        self.impl_mul_f32(rhs)
    }
}
impl ::std::ops::Mul<f32> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        self.impl_mul_f32(rhs)
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct Vector4 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl Vector4 {}
impl ::std::default::Default for Vector4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Vector4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Vector4").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
impl ::std::cmp::PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.W == other.W
    }
}
impl ::std::cmp::Eq for Vector4 {}
unsafe impl ::windows::runtime::Abi for Vector4 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Vector4 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4)");
}
impl Vector4 {
    pub fn new(X: f32, Y: f32, Z: f32, W: f32) -> Self {
        Self { X, Y, Z, W }
    }
    pub fn zero() -> Self {
        Self { X: 0f32, Y: 0f32, Z: 0f32, W: 0f32 }
    }
    pub fn one() -> Self {
        Self { X: 1f32, Y: 1f32, Z: 1f32, W: 1f32 }
    }
    pub fn unit_x() -> Self {
        Self { X: 1.0, Y: 0.0, Z: 0.0, W: 0.0 }
    }
    pub fn unit_y() -> Self {
        Self { X: 0.0, Y: 1.0, Z: 0.0, W: 0.0 }
    }
    pub fn unit_z() -> Self {
        Self { X: 0.0, Y: 0.0, Z: 1.0, W: 0.0 }
    }
    pub fn unit_w() -> Self {
        Self { X: 0.0, Y: 0.0, Z: 0.0, W: 1.0 }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.X * rhs.X + self.Y * rhs.Y + self.Z * rhs.Z + self.W * rhs.W
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self { X: self.X + rhs.X, Y: self.Y + rhs.Y, Z: self.Z + rhs.Z, W: self.W + rhs.W }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self { X: self.X - rhs.X, Y: self.Y - rhs.Y, Z: self.Z - rhs.Z, W: self.W - rhs.W }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self { X: self.X / rhs.X, Y: self.Y / rhs.Y, Z: self.Z / rhs.Z, W: self.W / rhs.W }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self { X: self.X / rhs, Y: self.Y / rhs, Z: self.Z / rhs, W: self.W / rhs }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self { X: self.X * rhs.X, Y: self.Y * rhs.Y, Z: self.Z * rhs.Z, W: self.W * rhs.W }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self { X: self.X * rhs, Y: self.Y * rhs, Z: self.Z * rhs, W: self.W * rhs }
    }
}
impl ::std::ops::Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Add<Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        self.impl_add(&rhs)
    }
}
impl ::std::ops::Add<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4 {
        self.impl_add(rhs)
    }
}
impl ::std::ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: &Vector4) -> Vector4 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Sub<Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        self.impl_sub(&rhs)
    }
}
impl ::std::ops::Sub<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: &Vector4) -> Vector4 {
        self.impl_sub(rhs)
    }
}
impl ::std::ops::Div<Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        self.impl_div(&rhs)
    }
}
impl ::std::ops::Div<&Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: &Vector4) -> Vector4 {
        self.impl_div(rhs)
    }
}
impl ::std::ops::Div<Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        self.impl_div(&rhs)
    }
}
impl ::std::ops::Div<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: &Vector4) -> Vector4 {
        self.impl_div(rhs)
    }
}
impl ::std::ops::Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Vector4 {
        self.impl_div_f32(rhs)
    }
}
impl ::std::ops::Div<f32> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Vector4 {
        self.impl_div_f32(rhs)
    }
}
impl ::std::ops::Mul<Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        self.impl_mul(&rhs)
    }
}
impl ::std::ops::Mul<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        self.impl_mul(rhs)
    }
}
impl ::std::ops::Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Vector4 {
        self.impl_mul_f32(rhs)
    }
}
impl ::std::ops::Mul<f32> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Vector4 {
        self.impl_mul_f32(rhs)
    }
}
