use ::util::Transform;
use ::intersectable::Intersectable;
use ::util::{Ray, IntersectionInfo};

pub struct Object {
  pub transform: Transform,
  pub intersectable: Box<dyn Intersectable>,
}

impl Object {
  pub fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
    let transf_ray = ray.inverse_transform(self.transform.into());
    self.intersectable.intersect(&transf_ray)
  }
}