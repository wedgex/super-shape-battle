use ggez::graphics::Mesh;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

mod drawable;
mod physicsable;
mod positionable;

pub trait Component: Any + Debug + Display {
  fn component_name() -> String
  where
    Self: Sized;
  fn name(&self) -> String
  where
    Self: std::marker::Sized,
  {
    Self::component_name()
  }
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
pub struct Drawable {
  mesh: Mesh,
}

impl Component for Drawable {
  fn component_name() -> String {
    "Drawable".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for Drawable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name(),)
  }
}

pub type EntityId = Uuid;

pub struct Entity {
  id: EntityId,
  components: HashMap<String, Box<dyn Component>>,
}

impl Entity {
  fn new() -> Self {
    Entity {
      id: Uuid::new_v4(),
      components: HashMap::new(),
    }
  }

  fn register_component<T: Component>(&mut self, component: T) {
    self
      .components
      .insert(component.name(), Box::new(component));
  }

  fn get_component<T: Component>(&mut self, name: &str) -> Option<&T> {
    if let Some(result) = self.components.get(name) {
      return result.as_any().downcast_ref::<T>();
    }

    None
  }
}

trait System {
  fn update();
}

#[cfg(test)]
mod tests {
  use super::drawable::Drawable;
  use super::physicsable::Physicsable;
  use super::positionable::Positionable;
  use super::*;
  use ggez::nalgebra::{Point2, Vector2};

  #[test]
  fn can_register_and_get_components() {
    let velocity = Physicsable::new(0., 0.);
    let position = Positionable::new(1., 2.);
    let mut entity = Entity::new();

    entity.register_component(velocity);
    entity.register_component(position);

    match entity.get_component::<Positionable>("Positionable") {
      Some(p) => assert_eq!(p.position, Point2::new(1., 2.)),
      None => assert!(false, "Positionable was not found"),
    }

    match entity.get_component::<Physicsable>("Physicsable") {
      Some(v) => assert_eq!(v.velocity, Vector2::new(0., 0.)),
      None => assert!(false, "Physicsable was not found"),
    }

    match entity.get_component::<Drawable>("Drawable") {
      Some(_) => assert!(false, "Unregistered Drawable found"),
      None => assert!(true),
    }
  }
}
