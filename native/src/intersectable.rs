use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn dot(&self, other: &Vector3) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Add<&Vector3> for &Vector3 {
  type Output = Vector3;

  fn add(self, rhs: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

impl Sub<&Vector3> for &Vector3 {
  type Output = Vector3;

  fn sub(self, rhs: &Vector3) -> Vector3 {
    Vector3 {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    }
  }
}

impl Mul<f32> for &Vector3 {
  type Output = Vector3;

  fn mul(self, rhs: f32) -> Vector3 {
    Vector3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    }
  }
}

impl Div<f32> for &Vector3 {
  type Output = Vector3;

  fn div(self, rhs: f32) -> Vector3 {
    Vector3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    }
  }
}

pub struct Ray {
  pub origin: Vector3,
  pub direction: Vector3,
}

impl Ray {
  pub fn point_at(&self, t: f32) -> Vector3 {
    &self.origin + &(&self.direction * t)
  }
}

pub struct IntersectionInfo {
  pub position: Vector3,
  pub normal: Vector3,
  pub t: f32,
}

pub trait Intersectable {
  fn intersect(self, ray: &Ray) -> Option<IntersectionInfo>;
}

pub struct Cube {
  pub size_x: f32, // x
  pub size_y: f32, // y
  pub size_z: f32,
}

pub struct Sphere {
  pub radius: f32,
}

pub struct Plane;

// impl Intersectable for Cube {
//   fn intersect(self, ray: &Ray) -> Option<IntersectionInfo> {

//   }
// }

impl Intersectable for Sphere {
  fn intersect(self, ray: &Ray) -> Option<IntersectionInfo> {
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&ray.origin);
    let c = ray.origin.dot(&ray.origin) - self.radius * self.radius;
    let d = (b * b - 4.0 * a * c).sqrt();
    let t1 = (-b + d) / (2.0 * a);
    let t2 = (-b - d) / (2.0 * a);
    let t = if t1 > 0.0 && t2 > 0.0 {
      t1.min(t2)
    } else if t1 * t2 < 0.0 {
      t1.max(t2)
    } else {
      return None
    };
    let position = ray.point_at(t);
    Some(IntersectionInfo {
      position: position.clone(),
      normal: position,
      t: t,
    })
  }
}

// impl Intersectable for Plane {
//   fn intersect(self, ray: &Ray) -> Option<IntersectionInfo> {

//   }
// }

pub struct Renderable {
  pub intersectable: Box<dyn Intersectable>,
}