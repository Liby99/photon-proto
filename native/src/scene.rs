use ::object::Object;
use ::util::{Ray, IntersectionInfo};

pub struct Scene {
  pub objects: Vec<Object>,
}

impl Scene {
  pub fn new() -> Self {
    Scene { objects: vec![] }
  }

  pub fn intersect(&self, ray: &Ray) -> Option<IntersectionInfo> {
    self.objects.iter().fold(None, |acc, obj| {
      IntersectionInfo::min(acc, obj.intersect(&ray))
    })
  }
}