use ::math::{Color, Vector3, Vector4, Quaternion, Matrix4};

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

  pub fn transform(&self, mat: Matrix4) -> Ray {
    Self {
      origin: self.origin.transform_dehomogenous(mat),
      direction: self.direction.transform(mat).normalize(),
    }
  }

  pub fn inverse_transform(&self, mat: Matrix4) -> Ray {
    self.transform(mat.inverse())
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

  pub fn transform(&self, mat: Matrix4) -> Self {
    Self {
      position: self.position.transform_dehomogenous(mat),
      normal: self.normal.transform(mat),
      t: self.t
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

#[derive(Copy, Clone)]
pub struct Transform {
  pub position: Vector3,
  pub scale: Vector3,
  pub rotation: Quaternion,
}

impl Into<Matrix4> for Transform {
  fn into(self) -> Matrix4 {
    let pos_mat = Matrix4::translate_matrix(self.position);
    let scale_mat = Matrix4::scale_matrix(self.scale);
    let rot_mat: Matrix4 = self.rotation.into();
    pos_mat * scale_mat * rot_mat
  }
}