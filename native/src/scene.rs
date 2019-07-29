use ::intersectable::Intersectable;

pub struct Scene {
  pub objects: Vec<Box<dyn Intersectable>>,
}

impl Scene {
  pub fn new() -> Self {
    Scene { objects: vec![] }
  }

  // pub fn add_object<T: Intersectable>(&mut self, obj: T) {
  //   self.objects.push(Box::new(obj));
  // }
}