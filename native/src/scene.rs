use ::object::Object;
use ::util::{Ray, Intersection};

#[derive(Clone)]
pub struct Scene {
  pub objects: Vec<Object>,
}

impl Scene {
  pub fn new() -> Self {
    Scene { objects: vec![] }
  }

  pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    self.objects.iter().fold(None, |acc, obj| {
      Intersection::min(acc, obj.intersect(&ray))
    })
  }
}