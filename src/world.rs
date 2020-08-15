use crate::components::Component;
use crate::components::{
  Collidable, Collision, ComponentManager, Damage, Damaged, Drawable, Expirable, Physicsable,
  PlayerControllable, Tag, Transform, Vulnerable,
};
use crate::entity::EntityId;
use crate::vec::intersections;
use std::any::TypeId;
use std::collections::HashMap;
use uuid::Uuid;

pub struct World {
  component_managers: HashMap<TypeId, ComponentManager>,
}

impl World {
  pub fn new() -> Self {
    let mut world = World {
      component_managers: HashMap::new(),
    };
    world.register::<Collidable>();
    world.register::<Collision>();
    world.register::<Damage>();
    world.register::<Damaged>();
    world.register::<Drawable>();
    world.register::<Expirable>();
    world.register::<Physicsable>();
    world.register::<PlayerControllable>();
    world.register::<Tag>();
    world.register::<Transform>();
    world.register::<Vulnerable>();

    world
  }

  pub fn create_entity(&self) -> EntityId {
    Uuid::new_v4()
  }

  pub fn register<T: Component>(&mut self) {
    self
      .component_managers
      .insert(TypeId::of::<T>(), ComponentManager::new());
  }

  pub fn get<T: Component>(&self, entity: &EntityId) -> Option<&T> {
    self
      .manager::<T>()
      .and_then(|manager| manager.get::<T>(entity))
  }

  pub fn get_mut<T: Component>(&mut self, entity: &EntityId) -> Option<&mut T> {
    self
      .manager_mut::<T>()
      .and_then(|manager| manager.get_mut::<T>(entity))
  }

  pub fn add<T: Component>(&mut self, entity: &EntityId, component: T) {
    self
      .manager_mut::<T>()
      .map(|manager| manager.add(entity, component));
  }

  pub fn remove(&mut self, entity: &EntityId) {
    // TODO keep track of which ones actually have an entity
    for manager in self.component_managers.values_mut() {
      manager.remove(entity);
    }
  }

  pub fn remove_all(&mut self, entities: Vec<EntityId>) {
    for entity in entities {
      self.remove(&entity);
    }
  }

  pub fn remove_component<T: Component>(&mut self, entity: &EntityId) {
    self
      .manager_mut::<T>()
      .map(|manager| manager.remove(entity));
  }

  pub fn components<T: Component>(&self) -> Vec<&T> {
    if let Some(manager) = self.manager::<T>() {
      return manager.components::<T>();
    }

    vec![]
  }

  pub fn entities<T: Component>(&self) -> Vec<EntityId> {
    if let Some(manager) = self.manager::<T>() {
      return manager.entities().into_iter().collect();
    }

    vec![]
  }

  pub fn entities_with(&self, components: Vec<TypeId>) -> Vec<EntityId> {
    let managers = components
      .iter()
      .filter_map(|c| self.component_managers.get(c));

    let entities = managers.map(|m| m.entities()).collect();

    intersections(entities)
  }

  fn manager<T: Component>(&self) -> Option<&ComponentManager> {
    self.component_managers.get(&TypeId::of::<T>())
  }

  fn manager_mut<T: Component>(&mut self) -> Option<&mut ComponentManager> {
    self.component_managers.get_mut(&TypeId::of::<T>())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod world {
    use super::*;
    use crate::components::Expirable;
    use crate::components::Transform;
    use ggez::nalgebra::Point2;
    use std::time::Duration;

    #[test]
    fn can_add_and_remove_components() {
      let mut world = World::new();

      let entity1 = world.create_entity();
      let entity2 = world.create_entity();

      let transform1 = Transform::new(0., 0.);
      let transform2 = Transform::new(1., 1.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));

      world.add(&entity1, transform1);
      world.add(&entity2, transform2);
      world.add(&entity1, expiration1);
      world.add(&entity2, expiration2);

      let transform1 = world.get::<Transform>(&entity1);
      assert!(transform1.is_some());
      assert_eq!(transform1.unwrap().position, Point2::new(0.0f32, 0.0f32));

      let transform2 = world.get::<Transform>(&entity2);
      assert!(transform2.is_some());
      assert_eq!(transform2.unwrap().position, Point2::new(1.0f32, 1.0f32));

      let expiration2 = world.get::<Expirable>(&entity2);
      assert!(expiration2.is_some());
      assert_eq!(expiration2.unwrap().expiration, Duration::from_secs(2));

      world.remove(&entity1);
      let transform1 = world.get::<Transform>(&entity1);
      assert!(transform1.is_none());

      world.remove(&entity2);
      let expiration2 = world.get::<Expirable>(&entity2);
      assert!(expiration2.is_none());
    }

    #[test]
    fn can_reference_entities_by_component_type() {
      let mut world = World::new();

      let entity1 = world.create_entity();
      let entity2 = world.create_entity();

      let transform1 = Transform::new(0., 0.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));

      let mut expected_transforms = vec![&entity1];
      expected_transforms.sort();
      let mut expected_expirations = vec![&entity1, &entity2];
      expected_expirations.sort();

      world.add(&entity1, transform1);
      world.add(&entity1, expiration1);
      world.add(&entity2, expiration2);

      let transforms = world.entities::<Transform>();
      let expirations = world.entities::<Expirable>();

      assert_eq!(transforms.len(), 1);
      assert_eq!(expirations.len(), 2);
    }

    #[test]
    fn can_reference_components_by_type() {
      let mut world = World::new();
      let entity1 = world.create_entity();
      let entity2 = world.create_entity();

      let transform1 = Transform::new(0., 0.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));
      let expected_points = vec![Point2::new(0.0f32, 0.0f32)];
      let expected_durations = vec![Duration::from_secs(1), Duration::from_secs(2)];

      world.add(&entity1, transform1);
      world.add(&entity1, expiration1);
      world.add(&entity2, expiration2);

      let transforms = world.components::<Transform>();
      let expirations = world.components::<Expirable>();
      let points: Vec<Point2<f32>> = transforms.iter().map(|t| t.position).collect();
      let durations: Vec<Duration> = expirations.iter().map(|t| t.expiration).collect();

      assert_eq!(transforms.len(), 1);
      assert_contains_all(points, expected_points);

      assert_eq!(expirations.len(), 2);
      assert_contains_all(durations, expected_durations);
    }

    #[test]
    fn can_reference_entities_with_multiple_components() {
      let mut world = World::new();

      let entity1 = world.create_entity();
      let entity2 = world.create_entity();

      let transform1 = Transform::new(0., 0.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));

      world.add(&entity1, transform1);
      world.add(&entity1, expiration1);
      world.add(&entity2, expiration2);

      let entities =
        world.entities_with(vec![TypeId::of::<Transform>(), TypeId::of::<Expirable>()]);

      assert_eq!(entities.len(), 1);
      assert_eq!(entities[0], entity1);
    }
  }

  fn assert_contains_all<T: std::cmp::PartialEq>(v1: Vec<T>, v2: Vec<T>) {
    for v in v2 {
      assert!(v1.contains(&v));
    }
  }
}
