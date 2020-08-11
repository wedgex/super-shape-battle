use crate::components::downcast_mut;
use crate::components::Component;
use crate::components::{
  downcast, Collidable, Collision, Damage, Damaged, Drawable, Expirable, Physicsable,
  PlayerControllable, Tag, Transform, Vulnerable,
};
use crate::entity::EntityId;
use std::any::TypeId;
use std::collections::HashMap;
use uuid::Uuid;

pub struct World {
  component_managers: HashMap<TypeId, ComponentManager>,
}

impl<'a> World {
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

  pub fn get<T: Component>(&self, eid: &EntityId) -> Option<&T> {
    self
      .manager::<T>()
      .and_then(|manager| manager.get::<T>(eid))
  }

  pub fn get_mut<T: Component>(&mut self, eid: &EntityId) -> Option<&mut T> {
    self
      .manager_mut::<T>()
      .and_then(|manager| manager.get_mut::<T>(eid))
  }

  pub fn add<T: Component>(&mut self, eid: &EntityId, component: T) {
    self
      .manager_mut::<T>()
      .map(|manager| manager.add(eid, component));
  }

  pub fn remove(&mut self, entity: &EntityId) {
    // TODO keep track of which ones actually have an entity
    for manager in self.component_managers.values_mut() {
      manager.remove(entity);
    }
  }

  pub fn remove_component<T: Component>(&mut self, eid: &EntityId) {
    self.manager_mut::<T>().map(|manager| manager.remove(eid));
  }

  pub fn components<T: Component>(&self) -> Vec<&T> {
    if let Some(manager) = self.manager::<T>() {
      return manager.components::<T>();
    }

    vec![]
  }

  pub fn entities<T: Component>(&self) -> Vec<&EntityId> {
    if let Some(manager) = self.manager::<T>() {
      return manager.entities().into_iter().collect();
    }

    vec![]
  }

  pub fn entities_with(&self, components: Vec<TypeId>) -> Vec<&EntityId> {
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

type SomeComponent = Box<dyn Component>;

struct ComponentManager {
  first_free: usize,
  entity_map: HashMap<EntityId, usize>,
  components: Vec<Option<SomeComponent>>,
}

impl<'a> ComponentManager {
  pub fn new() -> Self {
    ComponentManager {
      first_free: 0,
      entity_map: HashMap::new(),
      components: vec![],
    }
  }

  pub fn get<T: Component>(&self, eid: &EntityId) -> Option<&T> {
    if let Some(i) = self.entity_map.get(&eid) {
      if let Some(component) = &self.components[*i] {
        return downcast::<T>(component);
      }
    }

    None
  }

  pub fn get_mut<T: Component>(&mut self, eid: &EntityId) -> Option<&mut T> {
    if let Some(i) = self.entity_map.get(&eid) {
      if let Some(component) = &mut self.components[*i] {
        return downcast_mut::<T>(component);
      }
    }

    None
  }

  pub fn add<T: Component>(&mut self, eid: &EntityId, component: T) {
    let i = self.first_free;
    self.components.insert(i, Some(Box::new(component)));
    self.entity_map.insert(*eid, i);
    self.first_free = self
      .components
      .iter()
      .position(|c| c.is_none())
      .unwrap_or(self.components.len())
  }

  pub fn remove(&mut self, eid: &EntityId) {
    if let Some(i) = self.entity_map.remove(eid) {
      self.components[i] = None;
      if i < self.first_free {
        self.first_free = i;
      }
    }
  }

  pub fn components<T: Component>(&self) -> Vec<&T> {
    self
      .components
      .iter()
      .filter_map(|c| c.as_ref())
      .filter_map(downcast::<T>)
      .collect()
  }

  pub fn entities(&self) -> Vec<&EntityId> {
    self.entity_map.keys().collect()
  }
}

fn intersection<T: std::cmp::PartialEq>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
  let mut intersection: Vec<T> = vec![];

  for item in vec1.into_iter() {
    if vec2.contains(&item) {
      intersection.push(item);
    }
  }

  intersection
}

fn intersections<T: std::cmp::PartialEq>(mut sets: Vec<Vec<T>>) -> Vec<T> {
  match (sets.pop(), sets.pop()) {
    (Some(set1), Some(set2)) => {
      sets.push(intersection(set1, set2));
      return intersections(sets);
    }
    (Some(set1), None) => return set1,
    _ => return vec![],
  };
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
    use uuid::Uuid;

    #[test]
    fn can_add_and_remove_components() {
      let mut world = World::new();

      let eid1 = Uuid::new_v4();
      let eid2 = Uuid::new_v4();

      let transform1 = Transform::new(0., 0.);
      let transform2 = Transform::new(1., 1.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));

      world.add(&eid1, transform1);
      world.add(&eid2, transform2);
      world.add(&eid1, expiration1);
      world.add(&eid2, expiration2);

      let transform1 = world.get::<Transform>(&eid1);
      assert!(transform1.is_some());
      assert_eq!(transform1.unwrap().position, Point2::new(0.0f32, 0.0f32));

      let transform2 = world.get::<Transform>(&eid2);
      assert!(transform2.is_some());
      assert_eq!(transform2.unwrap().position, Point2::new(1.0f32, 1.0f32));

      let expiration2 = world.get::<Expirable>(&eid2);
      assert!(expiration2.is_some());
      assert_eq!(expiration2.unwrap().expiration, Duration::from_secs(2));

      world.remove(&eid1);
      let transform1 = world.get::<Transform>(&eid1);
      assert!(transform1.is_none());

      world.remove(&eid2);
      let expiration2 = world.get::<Expirable>(&eid2);
      assert!(expiration2.is_none());
    }

    #[test]
    fn can_reference_entities_by_component_type() {
      let mut world = World::new();

      let eid1 = Uuid::new_v4();
      let eid2 = Uuid::new_v4();

      let transform1 = Transform::new(0., 0.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));

      let mut expected_transforms = vec![&eid1];
      expected_transforms.sort();
      let mut expected_expirations = vec![&eid1, &eid2];
      expected_expirations.sort();

      world.add(&eid1, transform1);
      world.add(&eid1, expiration1);
      world.add(&eid2, expiration2);

      let transforms = world.entities::<Transform>();
      let expirations = world.entities::<Expirable>();

      assert_eq!(transforms.len(), 1);
      assert_eq!(expirations.len(), 2);
    }

    #[test]
    fn can_reference_components_by_type() {
      let mut world = World::new();
      let eid1 = Uuid::new_v4();
      let eid2 = Uuid::new_v4();

      let transform1 = Transform::new(0., 0.);
      let expiration1 = Expirable::new(Duration::from_secs(1));
      let expiration2 = Expirable::new(Duration::from_secs(2));
      let expected_points = vec![Point2::new(0.0f32, 0.0f32)];
      let expected_durations = vec![Duration::from_secs(1), Duration::from_secs(2)];

      world.add(&eid1, transform1);
      world.add(&eid1, expiration1);
      world.add(&eid2, expiration2);

      let transforms = world.components::<Transform>();
      let expirations = world.components::<Expirable>();

      assert_eq!(transforms.len(), 1);
      assert_eq!(
        transforms
          .iter()
          .map(|t| t.position)
          .collect::<Vec<Point2<f32>>>(),
        expected_points
      );
      assert_eq!(expirations.len(), 2);
      assert_eq!(
        expirations
          .iter()
          .map(|t| t.expiration)
          .collect::<Vec<Duration>>(),
        expected_durations
      );
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
      assert_eq!(*entities[0], entity1);
    }
  }

  mod component_manager {
    use super::*;
    use crate::components::Transform;
    use ggez::nalgebra::Point2;
    use uuid::Uuid;

    #[test]
    fn can_add_and_remove_components() {
      let transform1 = Transform::new(0., 0.);
      let transform2 = Transform::new(1., 1.);
      let eid1 = Uuid::new_v4();
      let eid2 = Uuid::new_v4();

      let mut component_manager = ComponentManager::new();

      component_manager.add(&eid1, transform1);
      component_manager.add(&eid2, transform2);

      let transform1 = component_manager.get::<Transform>(&eid1);
      assert!(transform1.is_some());
      assert_eq!(transform1.unwrap().position, Point2::new(0.0f32, 0.0f32));

      component_manager.remove(&eid1);
      let transform1 = component_manager.get::<Transform>(&eid1);
      assert!(transform1.is_none());

      let transform2 = component_manager.get::<Transform>(&eid2);
      assert!(transform2.is_some());
      assert_eq!(transform2.unwrap().position, Point2::new(1.0f32, 1.0f32));
    }

    #[test]
    fn can_reference_entity_ids() {
      let transform1 = Transform::new(0., 0.);
      let transform2 = Transform::new(1., 1.);
      let eid1 = Uuid::new_v4();
      let eid2 = Uuid::new_v4();
      let expected_eid1 = eid1.clone();
      let expected_eid2 = eid2.clone();
      let mut expected_entities = [&expected_eid1, &expected_eid2];
      expected_entities.sort();

      let mut component_manager = ComponentManager::new();

      component_manager.add(&eid1, transform1);
      component_manager.add(&eid2, transform2);

      let mut entities = component_manager.entities();
      entities.sort();

      assert_eq!(entities, expected_entities);
    }

    #[test]
    fn can_reference_components() {
      let transform1 = Transform::new(0., 0.);
      let transform2 = Transform::new(1., 1.);
      let eid1 = Uuid::new_v4();
      let eid2 = Uuid::new_v4();
      let expected_point1 = transform1.position.clone();
      let expected_point2 = transform2.position.clone();

      let mut component_manager = ComponentManager::new();

      component_manager.add(&eid1, transform1);
      component_manager.add(&eid2, transform2);

      let transforms: Vec<&Transform> = component_manager.components().into_iter().collect();

      assert_eq!(transforms.len(), 2);
      assert_eq!(transforms[0].position, expected_point1);
      assert_eq!(transforms[1].position, expected_point2);
    }
  }

  mod intersections {
    use super::*;

    #[test]
    fn intersection_empty_for_empty_vecs() {
      let v1: Vec<&str> = vec![];
      let v2: Vec<&str> = vec![];

      assert_eq!(intersection(v1, v2).len(), 0);
    }

    #[test]
    fn intersection_empty_without_overlap() {
      let v1: Vec<&str> = vec!["1", "2"];
      let v2: Vec<&str> = vec!["4", "3"];

      let result = intersection(v1, v2);

      assert_eq!(result.len(), 0);
    }

    #[test]
    fn intersection_with_overlap() {
      let v1: Vec<&str> = vec!["1", "3", "2"];
      let v2: Vec<&str> = vec!["2", "4", "3"];

      let result = intersection(v1, v2);

      assert_eq!(result.len(), 2);
      assert_eq!(result, vec!["3", "2"]);
    }

    #[test]
    fn intersections_with_no_overlap() {
      let sets = vec![vec!["1", "2"], vec!["3", "4"], vec!["5", "6"]];

      let result = intersections(sets);

      assert_eq!(result.len(), 0);
    }

    #[test]
    fn intersections_with_overlap() {
      let sets = vec![
        vec!["1", "2", "5"],
        vec!["3", "1", "4", "5"],
        vec!["5", "6", "1"],
      ];

      let result = intersections(sets);

      assert_eq!(result.len(), 2);
      assert_eq!(result, vec!["5", "1"]);
    }
  }
}
