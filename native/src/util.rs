use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
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

impl Into<Vector3> for Vector4 {
  fn into(self) -> Vector3 {
    Vector3 { x: self.x, y: self.y, z: self.z }
  }
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

impl Matrix4 {
  pub fn identity() -> Self {
    Self {
      a11: 1.0, a12: 0.0, a13: 0.0, a14: 0.0,
      a21: 0.0, a22: 1.0, a23: 0.0, a24: 0.0,
      a31: 0.0, a32: 0.0, a33: 1.0, a34: 0.0,
      a41: 0.0, a42: 0.0, a43: 0.0, a44: 1.0,
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

pub struct Ray {
  pub origin: Vector3,
  pub direction: Vector3,
}

impl Ray {
  pub fn new(origin: Vector3, direction: Vector3) -> Ray {
    Ray { origin, direction }
  }

  pub fn point_at(&self, t: f32) -> Vector3 {
    self.origin.clone() + self.direction.clone() * t
  }
}

#[derive(Clone, Copy)]
pub struct IntersectionInfo {
  pub position: Vector3,
  pub normal: Vector3,
  pub t: f32,
}

impl IntersectionInfo {
  pub fn min(lhs: Option<Self>, rhs: Option<Self>) -> Option<Self> {
    match (lhs, rhs) {
      (Some(i1), Some(i2)) => {
        if i1.t < i2.t {
          lhs.clone()
        } else {
          rhs.clone()
        }
      },
      _ => lhs.or(rhs)
    }
  }
}

pub struct ImageData<'a> {
  pub width: usize,
  pub height: usize,
  pub buffer: &'a mut [u8],
}

impl<'a> ImageData<'a> {
  pub fn set_pixel(&mut self, x: usize, y: usize, c: &Color) {
    let index = (y * self.width + x) * 4;
    self.buffer[index] = c.r;
    self.buffer[index + 1] = c.g;
    self.buffer[index + 2] = c.b;
    self.buffer[index + 3] = c.a;
  }
}

pub struct Transform {
  position: Vector3,
  scale: Vector3,
  rotation: Quaternion,
}

impl Into<Matrix4> for Transform {
  fn into(self) -> Matrix4 {
    let pos_mat = Matrix4::translate_matrix(self.position);
    let scale_mat = Matrix4::scale_matrix(self.scale);
    let rot_mat = self.rotation.into();
    pos_mat * scale_mat * rot_mat
  }
}