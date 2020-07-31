use std::any::Any;
use std::time::{Duration, Instant};

use super::Component;

pub struct Expirable {
  pub created: Instant,
  pub expiration: Duration,
}

impl Expirable {
  pub fn new(expiration: Duration) -> Self {
    Expirable {
      created: Instant::now(),
      expiration,
    }
  }
}

impl Component for Expirable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}