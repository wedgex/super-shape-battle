use std::any::Any;
use std::fmt;

use super::Component;

#[derive(PartialEq, Debug)]
pub enum TagType {
  Ship,
  Shape(u8),
  Bullet,
}

impl fmt::Display for TagType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let printable = match *self {
      TagType::Ship => String::from("Ship"),
      TagType::Shape(u) => format!("Shape({})", u),
      TagType::Bullet => String::from("Bullet"),
    };

    write!(f, "{}", printable)
  }
}

pub struct Tag {
  pub tag_type: TagType,
}

impl Tag {
  pub fn new(tag_type: TagType) -> Self {
    Tag { tag_type }
  }
}

impl Component for Tag {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
