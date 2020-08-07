use crate::components::Component;
use crate::entity::EntityId;
use std::any::TypeId;
use std::collections::HashMap;

struct World {
  component_managers: HashMap<TypeId, ComponentManager>,
}

impl World {
  pub fn new() -> Self {
    World {
      component_managers: HashMap::new(),
    }
  }

  pub fn register(&mut self, type_id: TypeId, manager: ComponentManager) {
    self.component_managers.insert(type_id, manager);
  }

  pub fn get<T: Component>(&self, eid: EntityId) -> Option<&T> {
    self
      .component_managers
      .get(&TypeId::of::<T>())
      .and_then(|manager| manager.get::<T>(eid))
  }

  pub fn add<T: Component>(&mut self, eid: EntityId, component: T) {
    self
      .component_managers
      .get_mut(&TypeId::of::<T>())
      .map(|manager| manager.add(eid, component));
  }

  pub fn remove<T: Component>(&mut self, eid: EntityId) {
    self
      .component_managers
      .get_mut(&TypeId::of::<T>())
      .map(|manager| manager.remove(eid));
  }
}

type SomeComponent = Box<dyn Component>;

struct ComponentManager {
  entity_map: HashMap<EntityId, usize>,
  components: Vec<Option<SomeComponent>>,
}

impl ComponentManager {
  pub fn new() -> Self {
    ComponentManager {
      entity_map: HashMap::new(),
      components: vec![],
    }
  }

  fn get<T: Component>(&self, eid: EntityId) -> Option<&T> {
    if let Some(i) = self.entity_map.get(&eid) {
      if let Some(component) = &self.components[*i] {
        return component.as_any().downcast_ref::<T>();
      }
    }

    None
  }

  fn add<T: Component>(&mut self, eid: EntityId, component: T) {
    // TODO memory compaction
    let i = self.components.len();
    self.components.push(Some(Box::new(component)));
    self.entity_map.insert(eid, i);
  }

  fn remove(&mut self, eid: EntityId) {
    if let Some(i) = self.entity_map.remove(&eid) {
      self.components[i] = None
    }
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

      let transform_manager = ComponentManager::new();
      let expirable_manager = ComponentManager::new();

      world.register(TypeId::of::<Transform>(), transform_manager);
      world.register(TypeId::of::<Expirable>(), expirable_manager);

      world.add(eid1, transform1);
      world.add(eid2, transform2);
      world.add(eid1, expiration1);
      world.add(eid2, expiration2);

      let transform1 = world.get::<Transform>(eid1);
      assert!(transform1.is_some());
      assert_eq!(transform1.unwrap().position, Point2::new(0.0f32, 0.0f32));

      let transform2 = world.get::<Transform>(eid2);
      assert!(transform2.is_some());
      assert_eq!(transform2.unwrap().position, Point2::new(1.0f32, 1.0f32));

      let expiration2 = world.get::<Expirable>(eid2);
      assert!(expiration2.is_some());
      assert_eq!(expiration2.unwrap().expiration, Duration::from_secs(2));

      world.remove::<Transform>(eid1);
      let transform1 = world.get::<Transform>(eid1);
      assert!(transform1.is_none());

      world.remove::<Expirable>(eid2);
      let expiration2 = world.get::<Expirable>(eid2);
      assert!(expiration2.is_none());
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

      component_manager.add(eid1, transform1);
      component_manager.add(eid2, transform2);

      let transform1 = component_manager.get::<Transform>(eid1);
      assert!(transform1.is_some());
      assert_eq!(transform1.unwrap().position, Point2::new(0.0f32, 0.0f32));

      component_manager.remove(eid1);
      let transform1 = component_manager.get::<Transform>(eid1);
      assert!(transform1.is_none());

      let transform2 = component_manager.get::<Transform>(eid2);
      assert!(transform2.is_some());
      assert_eq!(transform2.unwrap().position, Point2::new(1.0f32, 1.0f32));
      // remove
    }
  }
}
