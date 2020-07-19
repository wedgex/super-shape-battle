use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};
use std::time::{Duration, Instant};

use super::Component;

#[derive(Debug)]
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

impl Display for Expirable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Expirable")
  }
}
