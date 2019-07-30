use ::util::{Vector3, ImageData, Ray};

pub struct Camera {
  pub position: Vector3,
  pub forward: Vector3,
  pub up: Vector3,
  pub fovy: f32,
  pub focal_distance: f32,
  pub aperture: f32,
}

impl Camera {
  pub fn new(position: Vector3, forward: Vector3) -> Self {
    Camera {
      position,
      forward: forward.normalize(),
      up: Vector3::j(),
      fovy: std::f32::consts::PI / 3.0,
      focal_distance: 1.0,
      aperture: 0.01,
    }
  }

  pub fn new_with_target(position: Vector3, target: Vector3) -> Self {
    Self::new(position, target - position)
  }

  pub fn rays<'a>(&'a self, width: usize, height: usize) -> CameraRays<'a> {
    let w = self.forward; // front
    let u = w.cross(self.up); // right
    let v = u.cross(w); // up
    let b = -(self.fovy / 2.0).tan();
    let a = b * width as f32 / height as f32;
    let hw = width as f32 / 2.0;
    let hh = height as f32 / 2.0;
    CameraRays {
      camera: self,
      i: 0,
      j: 0,
      w: self.forward,
      u,
      v,
      b,
      a,
      hw,
      hh,
      width,
      height,
    }
  }
}

pub struct CameraRays<'a> {

  // Reference store
  camera: &'a Camera,

  // i and j indicating pixel in the image
  i: usize,
  j: usize,

  // Precomputation caches
  width: usize,
  height: usize,
  hw: f32,
  hh: f32,
  w: Vector3,
  u: Vector3,
  v: Vector3,
  a: f32,
  b: f32,
}

impl<'a> Iterator for CameraRays<'a> {
  type Item = (usize, usize, Ray);

  fn next(&mut self) -> Option<(usize, usize, Ray)> {

    // Get the next ray i and j
    let (new_i, new_j) = if self.i < self.width - 1 {
      (self.i + 1, self.j)
    } else if self.j < self.height - 1 {
      (0, self.j + 1)
    } else {
      return None;
    };

    // Mutate the state
    self.i = new_i;
    self.j = new_j;

    // Calculate ray
    let origin = self.camera.position;
    let hor_dir = self.u * self.a * (new_i as f32 - self.hw) / self.hw;
    let ver_dir = self.v * self.b * (new_j as f32 - self.hh) / self.hh;
    let direction = (self.w + hor_dir + ver_dir).normalize();
    let ray = Ray::new(origin, direction);

    // Has the next ray
    return Some((new_i, new_j, ray));
  }
}