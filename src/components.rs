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
struct PositionComponent {
  position: Point2<f32>,
}

impl PositionComponent {
  pub fn new(x: f32, y: f32) -> Self {
    PositionComponent {
      position: Point2::new(x, y),
    }
  }
}

impl Component for PositionComponent {
  fn name(&self) -> String {
    "PositionComponent".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for PositionComponent {
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
pub struct VelocityComponent {
  velocity: Vector2<f32>,
}

impl VelocityComponent {
  pub fn new(x: f32, y: f32) -> Self {
    VelocityComponent {
      velocity: Vector2::new(x, y),
    }
  }
}

impl Component for VelocityComponent {
  fn name(&self) -> String {
    "VelocityComponent".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for VelocityComponent {
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
pub struct DrawableComponent {
  mesh: Mesh,
}

impl Component for DrawableComponent {
  fn name(&self) -> String {
    "DrawableComponent".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for DrawableComponent {
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
    let velocity = VelocityComponent::new(0., 0.);
    let position = PositionComponent::new(1., 2.);
    let mut entity = Entity::new();

    entity.register_component(velocity);
    entity.register_component(position);

    match entity.get_component::<PositionComponent>("PositionComponent") {
      Some(p) => assert_eq!(p.position, Point2::new(1., 2.)),
      None => assert!(false, "PositionComponent was not found"),
    }

    match entity.get_component::<VelocityComponent>("VelocityComponent") {
      Some(v) => assert_eq!(v.velocity, Vector2::new(0., 0.)),
      None => assert!(false, "VelocityComponent was not found"),
    }

    match entity.get_component::<DrawableComponent>("DrawableComponent") {
      Some(_) => assert!(false, "Unregistered DrawableComponent found"),
      None => assert!(true),
    }
  }
}
