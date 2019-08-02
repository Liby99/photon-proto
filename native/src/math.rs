use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

fn f32_to_u8(num: f32) -> u8 {
  let num = if num > 1.0 { 1.0 } else if num < 0.0 { 0.0 } else { num };
  (num * 255.0) as u8
}

impl From<Vector4> for Color {
  fn from(v: Vector4) -> Color {
    Color {
      r: f32_to_u8(v.x),
      g: f32_to_u8(v.y),
      b: f32_to_u8(v.z),
      a: f32_to_u8(v.w),
    }
  }
}

impl From<Vector3> for Color {
  fn from(v: Vector3) -> Color {
    Color {
      r: f32_to_u8(v.x),
      g: f32_to_u8(v.y),
      b: f32_to_u8(v.z),
      a: 255,
    }
  }
}

impl Color {
  pub fn transparent() -> Color {
    Color { r: 0, g: 0, b: 0, a: 0 }
  }

  pub fn white() -> Color {
    Color { r: 255, g: 255, b: 255, a: 255 }
  }

  pub fn black() -> Color {
    Color { r: 0, g: 0, b: 0, a: 255 }
  }
}

#[derive(Clone, Copy)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn zero() -> Vector3 {
    Vector3 { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3 { x, y, z }
  }

  pub fn i() -> Vector3 {
    Vector3 { x: 1.0, y: 0.0, z: 0.0 }
  }

  pub fn j() -> Vector3 {
    Vector3 { x: 0.0, y: 1.0, z: 0.0 }
  }

  pub fn k() -> Vector3 {
    Vector3 { x: 0.0, y: 0.0, z: 1.0 }
  }

  pub fn dot(&self, other: &Vector3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }

  pub fn cross(&self, other: Vector3) -> Vector3 {
    Vector3 {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
    }
  }

  pub fn max(&self, other: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
    }
  }

  pub fn min(&self, other: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
    }
  }

  pub fn mag2(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }

  pub fn mag(&self) -> f32 {
    self.mag2().sqrt()
  }

  pub fn normalize(&self) -> Vector3 {
    let mag = self.mag();
    Vector3 { x: self.x / mag, y: self.y / mag, z: self.z / mag }
  }

  pub fn transform(self, mat: Matrix4) -> Vector3 {
    let vec4 = Vector4::vec3w(self, 0.0);
    let transf = mat * vec4;
    Vector3::from(transf)
  }

  pub fn transform_dehomogenous(self, mat: Matrix4) -> Vector3 {
    let vec4 = Vector4::vec3w(self, 1.0);
    let transf = mat * vec4;
    Vector3::from(transf) / transf.w
  }
}

impl From<Vector4> for Vector3 {
  fn from(vec: Vector4) -> Self {
    Self { x: vec.x, y: vec.y, z: vec.z }
  }
}

impl Neg for Vector3 {
  type Output = Self;

  fn neg(self) -> Self {
    Vector3 { x: -self.x, y: -self.y, z: -self.z }
  }
}

impl Add<Vector3> for Vector3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub<Vector3> for Vector3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl Mul<f32> for Vector3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Div<f32> for Vector3 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

impl Div<Vector3> for Vector3 {
  type Output = Self;

  fn div(self, rhs: Vector3) -> Self {
    Self {
      x: self.x / rhs.x,
      y: self.y / rhs.y,
      z: self.z / rhs.z,
    }
  }
}

#[derive(Clone, Copy)]
pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Vector4 {
  pub fn xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }

  pub fn vec3w(v: Vector3, w: f32) -> Self {
    Self { x: v.x, y: v.y, z: v.z, w }
  }

  pub fn zero() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
  }

  pub fn unit_x() -> Self {
    Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 }
  }

  pub fn unit_y() -> Self {
    Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 }
  }

  pub fn unit_z() -> Self {
    Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 }
  }

  pub fn unit_w() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
  }

  pub fn dot(self, other: Self) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }
}

pub type Quaternion = Vector4;

impl Into<Matrix4> for Quaternion {
  fn into(self) -> Matrix4 {
    Matrix4 {
      a11: 1.0 - 2.0 * self.y * self.y - 2.0 * self.z * self.z,
      a12: 2.0 * self.x * self.y - 2.0 * self.z * self.w,
      a13: 2.0 * self.x * self.z + 2.0 * self.y * self.w,
      a14: 0.0,
      a21: 2.0 * self.x * self.y + 2.0 * self.z * self.w,
      a22: 1.0 - 2.0 * self.x * self.x - 2.0 * self.z * self.z,
      a23: 2.0 * self.y * self.z - 2.0 * self.x * self.w,
      a24: 0.0,
      a31: 2.0 * self.x * self.z - 2.0 * self.y * self.w,
      a32: 2.0 * self.y * self.z + 2.0 * self.x * self.w,
      a33: 1.0 - 2.0 * self.x * self.x - 2.0 * self.y * self.y,
      a34: 0.0,
      a41: 0.0,
      a42: 0.0,
      a43: 0.0,
      a44: 1.0,
    }
  }
}

impl Quaternion {
  pub fn identity() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
  }

  pub fn axis_angle(axis: Vector3, angle: f32) -> Self {
    let s = (angle / 2.0).sin();
    Self {
      x: axis.x * s,
      y: axis.y * s,
      z: axis.z * s,
      w: (angle / 2.0).cos()
    }
  }
}

#[derive(Clone, Copy)]
pub struct Matrix4 {
  pub a11: f32, pub a12: f32, pub a13: f32, pub a14: f32,
  pub a21: f32, pub a22: f32, pub a23: f32, pub a24: f32,
  pub a31: f32, pub a32: f32, pub a33: f32, pub a34: f32,
  pub a41: f32, pub a42: f32, pub a43: f32, pub a44: f32,
}

impl Add<Matrix4> for Matrix4 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      a11: self.a11 + other.a11, a12: self.a12 + other.a12, a13: self.a13 + other.a13, a14: self.a14 + other.a14,
      a21: self.a21 + other.a21, a22: self.a22 + other.a22, a23: self.a23 + other.a23, a24: self.a24 + other.a24,
      a31: self.a31 + other.a31, a32: self.a32 + other.a32, a33: self.a33 + other.a33, a34: self.a34 + other.a34,
      a41: self.a41 + other.a41, a42: self.a42 + other.a42, a43: self.a43 + other.a43, a44: self.a44 + other.a44,
    }
  }
}

impl Mul<Matrix4> for Matrix4 {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    Self {
      a11: self.a11 * other.a11 + self.a12 * other.a21 + self.a13 * other.a31 + self.a14 * other.a41,
      a12: self.a11 * other.a12 + self.a12 * other.a22 + self.a13 * other.a32 + self.a14 * other.a42,
      a13: self.a11 * other.a13 + self.a12 * other.a23 + self.a13 * other.a33 + self.a14 * other.a43,
      a14: self.a11 * other.a14 + self.a12 * other.a24 + self.a13 * other.a34 + self.a14 * other.a44,

      a21: self.a21 * other.a11 + self.a22 * other.a21 + self.a23 * other.a31 + self.a24 * other.a41,
      a22: self.a21 * other.a12 + self.a22 * other.a22 + self.a23 * other.a32 + self.a24 * other.a42,
      a23: self.a21 * other.a13 + self.a22 * other.a23 + self.a23 * other.a33 + self.a24 * other.a43,
      a24: self.a21 * other.a14 + self.a22 * other.a24 + self.a23 * other.a34 + self.a24 * other.a44,

      a31: self.a31 * other.a11 + self.a32 * other.a21 + self.a33 * other.a31 + self.a34 * other.a41,
      a32: self.a31 * other.a12 + self.a32 * other.a22 + self.a33 * other.a32 + self.a34 * other.a42,
      a33: self.a31 * other.a13 + self.a32 * other.a23 + self.a33 * other.a33 + self.a34 * other.a43,
      a34: self.a31 * other.a14 + self.a32 * other.a24 + self.a33 * other.a34 + self.a34 * other.a44,

      a41: self.a41 * other.a11 + self.a42 * other.a21 + self.a43 * other.a31 + self.a44 * other.a41,
      a42: self.a41 * other.a12 + self.a42 * other.a22 + self.a43 * other.a32 + self.a44 * other.a42,
      a43: self.a41 * other.a13 + self.a42 * other.a23 + self.a43 * other.a33 + self.a44 * other.a43,
      a44: self.a41 * other.a14 + self.a42 * other.a24 + self.a43 * other.a34 + self.a44 * other.a44,
    }
  }
}

impl Mul<Vector4> for Matrix4 {
  type Output = Vector4;

  fn mul(self, other: Vector4) -> Vector4 {
    Vector4 {
      x: self.a11 * other.x + self.a12 * other.y + self.a13 * other.z + self.a14 * other.w,
      y: self.a21 * other.x + self.a22 * other.y + self.a23 * other.z + self.a24 * other.w,
      z: self.a31 * other.x + self.a32 * other.y + self.a33 * other.z + self.a34 * other.w,
      w: self.a41 * other.x + self.a42 * other.y + self.a43 * other.z + self.a44 * other.w,
    }
  }
}

impl Matrix4 {
  pub fn identity() -> Self {
    Self {
      a11: 1.0, a12: 0.0, a13: 0.0, a14: 0.0,
      a21: 0.0, a22: 1.0, a23: 0.0, a24: 0.0,
      a31: 0.0, a32: 0.0, a33: 1.0, a34: 0.0,
      a41: 0.0, a42: 0.0, a43: 0.0, a44: 1.0,
    }
  }

  pub fn transpose(self) -> Self {
    Self {
      a11: self.a11, a12: self.a21, a13: self.a31, a14: self.a41,
      a21: self.a12, a22: self.a22, a23: self.a32, a24: self.a42,
      a31: self.a13, a32: self.a23, a33: self.a33, a34: self.a43,
      a41: self.a14, a42: self.a24, a43: self.a34, a44: self.a44,
    }
  }

  pub fn inverse(self) -> Self {

    // Cache the value
    let a00 = self.a11;
    let a01 = self.a12;
    let a02 = self.a13;
    let a03 = self.a14;
    let a10 = self.a21;
    let a11 = self.a22;
    let a12 = self.a23;
    let a13 = self.a24;
    let a20 = self.a31;
    let a21 = self.a32;
    let a22 = self.a33;
    let a23 = self.a34;
    let a30 = self.a41;
    let a31 = self.a42;
    let a32 = self.a43;
    let a33 = self.a44;

    // Calculate...
    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;

    // Calculate the determinant
    let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
    if det == 0.0 {
      panic!("Matrix not invertible");
    }
    let det = 1.0 / det;

    // Final
    let o11 = (a11 * b11 - a12 * b10 + a13 * b09) * det;
    let o12 = (a02 * b10 - a01 * b11 - a03 * b09) * det;
    let o13 = (a31 * b05 - a32 * b04 + a33 * b03) * det;
    let o14 = (a22 * b04 - a21 * b05 - a23 * b03) * det;
    let o21 = (a12 * b08 - a10 * b11 - a13 * b07) * det;
    let o22 = (a00 * b11 - a02 * b08 + a03 * b07) * det;
    let o23 = (a32 * b02 - a30 * b05 - a33 * b01) * det;
    let o24 = (a20 * b05 - a22 * b02 + a23 * b01) * det;
    let o31 = (a10 * b10 - a11 * b08 + a13 * b06) * det;
    let o32 = (a01 * b08 - a00 * b10 - a03 * b06) * det;
    let o33 = (a30 * b04 - a31 * b02 + a33 * b00) * det;
    let o34 = (a21 * b02 - a20 * b04 - a23 * b00) * det;
    let o41 = (a11 * b07 - a10 * b09 - a12 * b06) * det;
    let o42 = (a00 * b09 - a01 * b07 + a02 * b06) * det;
    let o43 = (a31 * b01 - a30 * b03 - a32 * b00) * det;
    let o44 = (a20 * b03 - a21 * b01 + a22 * b00) * det;

    Self {
      a11: o11, a12: o12, a13: o13, a14: o14,
      a21: o21, a22: o22, a23: o23, a24: o24,
      a31: o31, a32: o32, a33: o33, a34: o34,
      a41: o41, a42: o42, a43: o43, a44: o44,
    }
  }

  pub fn scale_matrix(scale: Vector3) -> Self {
    Self {
      a11: scale.x, a12: 0.0, a13: 0.0, a14: 0.0,
      a21: 0.0, a22: scale.y, a23: 0.0, a24: 0.0,
      a31: 0.0, a32: 0.0, a33: scale.z, a34: 0.0,
      a41: 0.0, a42: 0.0, a43: 0.0, a44: 1.0,
    }
  }

  pub fn translate_matrix(position: Vector3) -> Self {
    Self {
      a11: 1.0, a12: 0.0, a13: 0.0, a14: position.x,
      a21: 0.0, a22: 1.0, a23: 0.0, a24: position.y,
      a31: 0.0, a32: 0.0, a33: 1.0, a34: position.z,
      a41: 0.0, a42: 0.0, a43: 0.0, a44: 1.0,
    }
  }
}