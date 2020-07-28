use super::Component;
use std::any::Any;
use std::time::{Duration, Instant};

pub struct PlayerControllable {
  pub last_fired: Instant,
}

impl PlayerControllable {
  pub fn new() -> Self {
    PlayerControllable {
      last_fired: Instant::now() - Duration::from_secs(5),
    }
  }
}

impl Component for PlayerControllable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
