use crate::entity::EntityId;
use std::any::Any;

use super::Component;

pub struct Collision {
  pub entity1: EntityId,
  pub entity2: EntityId,
}

impl Collision {
  pub fn new(entity1: EntityId, entity2: EntityId) -> Self {
    Collision { entity1, entity2 }
  }
}

impl Component for Collision {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
