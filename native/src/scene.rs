use ::intersectable::Intersectable;

pub struct Scene<T: Intersectable> {
  pub objects: Vec<Box<T>>,
}

impl<T: Intersectable> Scene<T> {
  pub fn new() -> Self {
    Scene { objects: vec![] }
  }

  pub fn add_object(&mut self, obj: T) {
    self.objects.push(Box::new(obj));
  }
}