use crate::entity::EntityId;
use std::any::Any;
use std::collections::HashMap;

mod collidable;
mod collision;
mod damage;
mod damaged;
mod drawable;
mod expirable;
mod physicsable;
mod player_controllable;
mod tag;
mod transform;
mod vulnerable;

pub use collidable::{Collidable, CollisionBounds};
pub use collision::Collision;
pub use damage::{Damage, DamageType};
pub use damaged::Damaged;
pub use drawable::Drawable;
pub use expirable::Expirable;
pub use physicsable::Physicsable;
pub use player_controllable::PlayerControllable;
pub use tag::{Tag, TagType};
pub use transform::Transform;
pub use vulnerable::Vulnerable;

pub trait Component: Any + 'static {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

type SomeComponent = Box<dyn Component>;

pub struct ComponentManager {
  entity_map: HashMap<EntityId, SomeComponent>,
}

impl ComponentManager {
  pub fn new() -> Self {
    ComponentManager {
      entity_map: HashMap::new(),
    }
  }

  pub fn get<T: Component>(&self, entity: &EntityId) -> Option<&T> {
    if let Some(component) = self.entity_map.get(&entity) {
      return downcast::<T>(component);
    }

    None
  }

  pub fn get_mut<T: Component>(&mut self, entity: &EntityId) -> Option<&mut T> {
    if let Some(component) = self.entity_map.get_mut(&entity) {
      return downcast_mut::<T>(component);
    }

    None
  }

  pub fn add<T: Component>(&mut self, entity: &EntityId, component: T) {
    self.entity_map.insert(*entity, Box::new(component));
  }

  pub fn remove(&mut self, entity: &EntityId) {
    self.entity_map.remove(entity);
  }

  pub fn components<T: Component>(&self) -> Vec<&T> {
    self.entity_map.values().filter_map(downcast::<T>).collect()
  }

  pub fn entities(&self) -> Vec<EntityId> {
    self.entity_map.keys().cloned().collect()
  }
}

fn downcast<T: Component>(c: &Box<dyn Component>) -> Option<&T> {
  c.as_any().downcast_ref::<T>()
}

fn downcast_mut<T: Component>(c: &mut Box<dyn Component>) -> Option<&mut T> {
  c.as_any_mut().downcast_mut::<T>()
}

#[cfg(test)]

mod tests {
  use super::*;
  use crate::components::Transform;
  use ggez::nalgebra::Point2;
  use uuid::Uuid;

  #[test]
  fn can_add_and_remove_components() {
    let transform1 = Transform::new(0., 0.);
    let transform2 = Transform::new(1., 1.);
    let entity1 = Uuid::new_v4();
    let entity2 = Uuid::new_v4();

    let mut component_manager = ComponentManager::new();

    component_manager.add(&entity1, transform1);
    component_manager.add(&entity2, transform2);

    let transform1 = component_manager.get::<Transform>(&entity1);
    assert!(transform1.is_some());
    assert_eq!(transform1.unwrap().position, Point2::new(0.0f32, 0.0f32));

    component_manager.remove(&entity1);
    let transform1 = component_manager.get::<Transform>(&entity1);
    assert!(transform1.is_none());

    let transform2 = component_manager.get::<Transform>(&entity2);
    assert!(transform2.is_some());
    assert_eq!(transform2.unwrap().position, Point2::new(1.0f32, 1.0f32));
  }

  #[test]
  fn can_reference_entity_ids() {
    let transform1 = Transform::new(0., 0.);
    let transform2 = Transform::new(1., 1.);
    let entity1 = Uuid::new_v4();
    let entity2 = Uuid::new_v4();
    let expected_entities = [entity1.clone(), entity2.clone()].to_vec();

    let mut component_manager = ComponentManager::new();

    component_manager.add(&entity1, transform1);
    component_manager.add(&entity2, transform2);

    let entities = component_manager.entities();

    assert_contains_all(entities, expected_entities);
  }

  #[test]
  fn can_reference_components() {
    let transform1 = Transform::new(0., 0.);
    let transform2 = Transform::new(1., 1.);
    let entity1 = Uuid::new_v4();
    let entity2 = Uuid::new_v4();
    let expected_point1 = transform1.position.clone();
    let expected_point2 = transform2.position.clone();

    let mut component_manager = ComponentManager::new();

    component_manager.add(&entity1, transform1);
    component_manager.add(&entity2, transform2);

    let transforms: Vec<&Transform> = component_manager.components().into_iter().collect();

    assert_eq!(transforms.len(), 2);
    assert_eq!(transforms[0].position, expected_point1);
    assert_eq!(transforms[1].position, expected_point2);
  }

  fn assert_contains_all<T: std::cmp::PartialEq>(v1: Vec<T>, v2: Vec<T>) {
    for v in v2 {
      assert!(v1.contains(&v));
    }
  }
}
