use ::util::{Vector3, Ray, IntersectionInfo};

pub trait Intersectable {
  fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo>;
}

pub struct Cube {
  pub size_x: f32,
  pub size_y: f32,
  pub size_z: f32,
}

impl Cube {
  pub fn new(size_x: f32, size_y: f32, size_z: f32) -> Cube {
    Self { size_x, size_y, size_z }
  }
}

impl Intersectable for Cube {
  fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
    let hx = self.size_x / 2.0;
    let hy = self.size_y / 2.0;
    let hz = self.size_z / 2.0;
    let min_corner = Vector3 { x: -hx, y: -hy, z: -hz };
    let max_corner = Vector3 { x: hx, y: hy, z: hz };
    let t_min_tmp = (min_corner - ray.origin) / ray.direction;
    let t_max_tmp = (max_corner - ray.origin) / ray.direction;
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

pub struct Plane;

impl Plane {
  pub fn new() -> Self {
    Self
  }
}

impl Intersectable for Plane {
  fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
    if ray.direction.y == 0.0 {
      return None;
    }
    let t = ray.origin.y / -ray.direction.y;
    if t > 0.0 {
      Some(IntersectionInfo {
        position: ray.point_at(t),
        normal: if ray.origin.y > 0.0 { Vector3::j() } else { -Vector3::j() },
        t
      })
    } else {
      None
    }
  }
}

pub struct Sphere {
  pub radius: f32,
}

impl Sphere {
  pub fn new(radius: f32) -> Self {
    Self { radius }
  }
}

impl Intersectable for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * ray.direction.dot(&ray.origin);
    let c = ray.origin.dot(&ray.origin) - self.radius * self.radius;
    let d = (b * b - 4.0 * a * c).sqrt();
    let t1 = (-b + d) / (2.0 * a);
    let t2 = (-b - d) / (2.0 * a);
    let (t, sign) = if t1 > 0.0 && t2 > 0.0 {
      (t1.min(t2), 1.0)
    } else if t1 * t2 < 0.0 {
      (t1.max(t2), -1.0)
    } else {
      return None
    };
    let position = ray.point_at(t);
    Some(IntersectionInfo {
      position: position,
      normal: (position * sign).normalize(),
      t: t,
    })
  }
}