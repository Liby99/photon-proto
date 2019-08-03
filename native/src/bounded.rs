use ::intersectable::{Cube, Sphere};
use ::math::{Vector3, Matrix4};

pub struct BoundingBox {
  pub min: Vector3,
  pub max: Vector3,
}

impl BoundingBox {
  pub fn new(min: Vector3, max: Vector3) -> Self {
    BoundingBox { min, max }
  }

  pub fn transform(&self, mat: Matrix4) -> Self {
    let pos = Vector3::from(mat.row(3));
    let mut bb = Self::new(pos, pos);
    for i in 0..3 {
      for j in 0..3 {
        let x = mat[(i, j)] * self.min[j];
        let y = mat[(i, j)] * self.max[j];
        if x < y {
          bb.min[i] += x;
          bb.max[i] += y;
        } else {
          bb.min[i] += y;
          bb.max[i] += x;
        }
      }
    }
    bb
  }
}

pub trait Bounded {
  fn bounding_box(&self) -> BoundingBox;
}

impl Bounded for Cube {
  fn bounding_box(&self) -> BoundingBox {
    let hx = self.size_x / 2.0;
    let hy = self.size_y / 2.0;
    let hz = self.size_z / 2.0;
    BoundingBox::new(Vector3::new(-hx, -hy, -hz), Vector3::new(hx, hy, hz))
  }
}

impl Bounded for Sphere {
  fn bounding_box(&self) -> BoundingBox {
    let v = Vector3::new(self.radius, self.radius, self.radius);
    BoundingBox::new(-v, v)
  }
}