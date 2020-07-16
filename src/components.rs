use ggez::graphics::Mesh;
use ggez::nalgebra::{Point2, Vector2};
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};
use uuid::Uuid;

pub trait Component: Any + Debug + Display {
  fn name(&self) -> String;
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Positionable {
  position: Point2<f32>,
}

impl Positionable {
  pub fn new(x: f32, y: f32) -> Self {
    Positionable {
      position: Point2::new(x, y),
    }
  }
}

impl Component for Positionable {
  fn name(&self) -> String {
    "Positionable".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for Positionable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}: (x: {}, y: {})",
      self.name(),
      self.position.x,
      self.position.y
    )
  }
}

#[derive(Debug)]
pub struct Velocity {
  velocity: Vector2<f32>,
}

impl Velocity {
  pub fn new(x: f32, y: f32) -> Self {
    Velocity {
      velocity: Vector2::new(x, y),
    }
  }
}

impl Component for Velocity {
  fn name(&self) -> String {
    "Velocity".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for Velocity {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}: (x: {}, y: {})",
      self.name(),
      self.velocity.x,
      self.velocity.y
    )
  }
}

#[derive(Debug)]
pub struct Drawable {
  mesh: Mesh,
}

impl Component for Drawable {
  fn name(&self) -> String {
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
  use super::*;

  #[test]
  fn can_register_and_get_components() {
    let velocity = Velocity::new(0., 0.);
    let position = Positionable::new(1., 2.);
    let mut entity = Entity::new();

    entity.register_component(velocity);
    entity.register_component(position);

    match entity.get_component::<Positionable>("Positionable") {
      Some(p) => assert_eq!(p.position, Point2::new(1., 2.)),
      None => assert!(false, "Positionable was not found"),
    }

    match entity.get_component::<Velocity>("Velocity") {
      Some(v) => assert_eq!(v.velocity, Vector2::new(0., 0.)),
      None => assert!(false, "Velocity was not found"),
    }

    match entity.get_component::<Drawable>("Drawable") {
      Some(_) => assert!(false, "Unregistered Drawable found"),
      None => assert!(true),
    }
  }
}
