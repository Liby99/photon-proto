use anymap::AnyMap;

struct Entity {
  pub components: AnyMap,
  pub children: Vec<Entity>,
}

impl Entity {
  pub fn new() -> Self {
    Entity {
      components: AnyMap::new(),
      children: vec![],
    }
  }
}

struct Scene {
  pub root: Entity,
}