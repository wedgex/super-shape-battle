use crate::components::DamageType;
use std::any::Any;

use super::Component;

pub struct Vulnerable {
  pub damage_types: Vec<DamageType>,
}

impl Vulnerable {
  pub fn new(damage_types: Vec<DamageType>) -> Self {
    Vulnerable { damage_types }
  }
}

impl Component for Vulnerable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
