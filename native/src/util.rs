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

pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

pub struct Matrix4 {

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