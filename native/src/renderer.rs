use ::util::{Vector3, Color};
use ::scene::Scene;
use ::intersectable::{Ray, Intersectable};

pub struct RayTracer;

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

impl RayTracer {
  pub fn render<T: Intersectable>(scene: &Scene<T>, img_data: &mut ImageData) {
    let cam_pos = Vector3::new(0.0, 0.0, 3.0);
    for i in 0..img_data.width {
      for j in 0..img_data.height {

        // Get the ray
        let x = -4.0 + 8.0 * (i as f32) / (img_data.width as f32);
        let y = -2.25 + 4.5 * (j as f32) / (img_data.height as f32);
        let target = Vector3::new(x, y, 0.0);
        let direction = (target - cam_pos.clone()).normalize();
        let ray = Ray { origin: cam_pos.clone(), direction };

        // Check if intersect with the scene objects
        for obj in scene.objects.iter() {
          let maybe_itsct = obj.intersect(&ray);

          // Get color
          let color = match maybe_itsct {
            Some(_) => Color::white(), // TODO: Change the colors
            None => Color::black()
          };

          // Set the pixel in the img_data
          img_data.set_pixel(i, j, &color);
        }
      }
    }
  }
}