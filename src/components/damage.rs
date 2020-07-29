use std::any::Any;

use super::Component;

#[derive(PartialEq)]
pub enum DamageType {
  Projectile,
  Smash,
}

pub struct Damage {
  pub damage_type: DamageType,
}

impl Damage {
  pub fn new(damage_type: DamageType) -> Self {
    Damage { damage_type }
  }
}

impl Component for Damage {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
