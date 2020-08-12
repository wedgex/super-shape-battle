use crate::entity::EntityId;
use std::any::Any;

use super::Component;

#[derive(Clone)]
pub struct Damaged {
  pub entity: EntityId,
}

impl Damaged {
  pub fn new(entity: EntityId) -> Self {
    Damaged { entity }
  }
}

impl Component for Damaged {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
