use ::util::{Vector3, Color, ImageData, Ray, IntersectionInfo};
use ::scene::Scene;
use ::intersectable::Intersectable;

pub struct RayTracer;

impl RayTracer {
  pub fn render(scene: &Scene, img_data: &mut ImageData) {
    let cam_pos = Vector3::new(0.0, 0.2, 3.0);
    for i in 0..img_data.width {
      for j in 0..img_data.height {

        // Get the ray
        let x = -4.0 + 8.0 * (i as f32) / (img_data.width as f32);
        let y = 2.25 - 4.5 * (j as f32) / (img_data.height as f32);
        let target = Vector3::new(x, y, 0.0);
        let direction = (target - cam_pos).normalize();
        let ray = Ray { origin: cam_pos, direction };

        // Check if intersect with the scene objects
        let maybe_itsct = scene.objects.iter().fold(None, |acc, obj| {
          IntersectionInfo::min(acc, obj.intersect(&ray))
        });

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