use crate::components::Component;
use uuid::Uuid;

pub type EntityId = Uuid;

pub struct Entity {
  pub id: EntityId,
  components: Vec<Box<dyn Component>>,
}

impl Entity {
  pub fn new() -> Self {
    Entity {
      id: Uuid::new_v4(),
      components: vec![],
    }
  }

  pub fn register_component<T: Component>(&mut self, component: T) {
    self.components.push(Box::new(component));
  }

  pub fn get_component<T: Component>(&self) -> Option<&T> {
    for component in self.components.iter() {
      if let Some(result) = component.as_any().downcast_ref::<T>() {
        return Some(result);
      }
    }

    None
  }

  pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
    for component in self.components.iter_mut() {
      if let Some(result) = component.as_any_mut().downcast_mut::<T>() {
        return Some(result);
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::components::Drawable;
  use crate::components::Physicsable;
  use crate::components::Positionable;
  use ggez::nalgebra::{Point2, Vector2};

  #[test]
  fn can_register_and_get_components() {
    let velocity = Physicsable::new(0., 0.);
    let position = Positionable::new(1., 2.);
    let mut entity = Entity::new();

    entity.register_component(velocity);
    entity.register_component(position);

    match entity.get_component::<Positionable>() {
      Some(p) => assert_eq!(p.position, Point2::new(1., 2.)),
      None => assert!(false, "Positionable was not found"),
    }

    match entity.get_component::<Physicsable>() {
      Some(v) => assert_eq!(v.velocity, Vector2::new(0., 0.)),
      None => assert!(false, "Physicsable was not found"),
    }

    match entity.get_component::<Drawable>() {
      Some(_) => assert!(false, "Unregistered Drawable found"),
      None => assert!(true),
    }
  }

  #[test]
  fn can_register_and_get_mut_components() {
    let velocity = Physicsable::new(0., 0.);
    let position = Positionable::new(1., 2.);
    let mut entity = Entity::new();

    entity.register_component(velocity);
    entity.register_component(position);

    match entity.get_component_mut::<Positionable>() {
      Some(p) => assert_eq!(p.position, Point2::new(1., 2.)),
      None => assert!(false, "Positionable was not found"),
    }

    match entity.get_component_mut::<Physicsable>() {
      Some(v) => assert_eq!(v.velocity, Vector2::new(0., 0.)),
      None => assert!(false, "Physicsable was not found"),
    }

    match entity.get_component_mut::<Drawable>() {
      Some(_) => assert!(false, "Unregistered Drawable found"),
      None => assert!(true),
    }
  }
}
