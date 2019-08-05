use ::intersectable::Intersectable;
use ::util::{Transform, Ray, Intersection};

#[derive(Clone)]
pub struct Object {
  pub transform: Transform,
  pub intersectable: Box<dyn Intersectable + Send>,
}

impl Object {
  pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
    let transf = self.transform.into();
    let transf_ray = ray.inverse_transform(transf);
    let maybe_itsct = self.intersectable.intersect(&transf_ray);
    maybe_itsct.map(|itsct| itsct.transform(transf))
  }
}