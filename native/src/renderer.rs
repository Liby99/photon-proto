use ::util::{Color, ImageData, IntersectionInfo};
use ::scene::Scene;
use ::camera::Camera;
// use ::intersectable::Intersectable;

pub struct RayTracer;

impl RayTracer {
  pub fn render(scene: &Scene, camera: &Camera, img_data: &mut ImageData) {
    for (i, j, ray) in camera.rays(img_data.width, img_data.height) {
      let color = match scene.intersect(&ray) {
        Some(_) => Color::white(), // TODO: Change the colors
        None => Color::black()
      };
      img_data.set_pixel(i, j, &color);
    }
  }
}