use crate::entity::EntityId;
use std::any::Any;

use super::Component;

pub struct Damaged {
  pub entity_id: EntityId,
}

impl Damaged {
  pub fn new(entity_id: EntityId) -> Self {
    Damaged { entity_id }
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
