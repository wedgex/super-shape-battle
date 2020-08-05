use crate::components::Component;
use crate::entity::EntityId;
use crate::systems::CollisionSystem;
use crate::systems::DamageSystem;
use crate::systems::DrawSystem;
use crate::systems::ExpirationSystem;
use crate::systems::PhysicsSystem;
use crate::systems::PlayerInputSystem;
use crate::systems::ShapeManager;
use crate::systems::ShipManager;
use ggez::event;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use crate::entity::Entity;
use crate::shape::{hexagon, octagon, square};
use crate::ship::build_ship;
use crate::systems::System;

pub struct GameState {
  pub entities: Vec<Entity>,
}

impl GameState {
  pub fn new(context: &mut Context) -> GameResult<GameState> {
    let octagon = octagon(context, 100.0, 100.0)?;
    let hexagon = hexagon(context, 500.0, 500.0)?;
    let square = square(context, 300.0, 200.0)?;

    let ship = build_ship(context)?;

    let s = GameState {
      entities: vec![ship, octagon, hexagon, square],
    };

    Ok(s)
  }

  pub fn entities_with<T: Component>(&self) -> Vec<&Entity> {
    self
      .entities
      .iter()
      .filter(|e| e.has_component::<T>())
      .collect()
  }

  pub fn entities_with_mut<T: Component>(&mut self) -> Vec<&mut Entity> {
    self
      .entities
      .iter_mut()
      .filter(|e| e.has_component::<T>())
      .collect()
  }

  pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
    self.entities.iter().find(|e| e.id == id)
  }

  pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
    self.entities.iter_mut().find(|e| e.id == id)
  }

  pub fn get_component<T: Component>(&self, id: EntityId) -> Option<&T> {
    self.get_entity(id).and_then(|e| e.get_component::<T>())
  }

  pub fn get_component_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T> {
    self
      .get_entity_mut(id)
      .and_then(|e| e.get_component_mut::<T>())
  }

  pub fn get_components<T: Component>(&self) -> Vec<&T> {
    self
      .entities
      .iter()
      .filter_map(|e| e.get_component::<T>())
      .collect()
  }

  pub fn get_components_mut<T: Component>(&mut self) -> Vec<&mut T> {
    self
      .entities
      .iter_mut()
      .filter_map(|e| e.get_component_mut::<T>())
      .collect()
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    PlayerInputSystem::update(self, ctx)?;
    PhysicsSystem::update(self, ctx)?;
    ExpirationSystem::update(self, ctx)?;
    CollisionSystem::update(self, ctx)?;
    DamageSystem::update(self, ctx)?;
    ShipManager::update(self, ctx)?;
    ShapeManager::update(self, ctx)?;

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);

    DrawSystem::update(self, ctx)?;

    graphics::present(ctx)?;

    Ok(())
  }
}
