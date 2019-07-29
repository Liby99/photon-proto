use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone)]
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

pub struct Ray {
  pub origin: Vector3,
  pub direction: Vector3,
}

impl Ray {
  pub fn point_at(&self, t: f32) -> Vector3 {
    self.origin.clone() + self.direction.clone() * t
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

impl Intersectable for Cube {
  fn intersect(self, ray: &Ray) -> Option<IntersectionInfo> {
    let hx = self.size_x / 2.0;
    let hy = self.size_y / 2.0;
    let hz = self.size_z / 2.0;
    let min_corner = Vector3 { x: -hx, y: -hy, z: -hz };
    let max_corner = Vector3 { x: hx, y: hy, z: hz };
    let t_min_tmp = (min_corner - ray.origin.clone()) / ray.direction.clone();
    let t_max_tmp = (max_corner - ray.origin.clone()) / ray.direction.clone();
    let t_min_vec = t_min_tmp.min(&t_max_tmp);
    let t_max_vec = t_min_tmp.max(&t_max_tmp);
    let t_min = t_min_vec.x.max(t_min_vec.y).max(t_min_vec.z);
    let t_max = t_max_vec.x.min(t_max_vec.y).min(t_max_vec.z);
    if t_max - t_min < 0.0 {
      return None;
    } else {
      let (t, sign) = if t_min > 0.0 && t_max > 0.0 {
        (t_min, 1.0)
      } else if t_min < 0.0 && t_max > 0.0 {
        (t_max, -1.0)
      } else {
        return None;
      };
      let position = ray.point_at(t);
      let normal = if t == t_min_vec.y || t == t_max_vec.y {
        if position.y > 0.0 { Vector3::j() } else { -Vector3::j() }
      } else if t == t_min_vec.x || t == t_min_vec.x {
        if position.x > 0.0 { Vector3::i() } else { -Vector3::i() }
      } else {
        if position.z > 0.0 { Vector3::k() } else { -Vector3::k() }
      } * sign;
      Some(IntersectionInfo { position, normal, t })
    }
  }
}

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