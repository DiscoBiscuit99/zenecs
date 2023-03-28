mod components;
mod entities;

use components::{Component, ComponentManager, DenseVecStorage};
use entities::{EntityBuilder, EntityId, EntityManager};

pub struct World {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn register<C: Component + 'static>(&mut self) {
        self.component_manager.register::<C, DenseVecStorage<C>>();
    }

    pub fn get<C: Component + 'static>(&self, entity: EntityId) -> Option<&C> {
        self.component_manager.get(entity)
    }

    pub fn spawn_entity(&mut self) -> EntityBuilder {
        EntityBuilder::new(self)
    }
}

//-- Tests --//

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Position {
        x: f32,
        y: f32,
    }

    #[derive(Debug, PartialEq)]
    struct Velocity {
        x: f32,
        y: f32,
    }

    impl Component for Position {
        type Storage = DenseVecStorage<Self>;
    }

    impl Component for Velocity {
        type Storage = DenseVecStorage<Self>;
    }

    #[test]
    fn world() {
        let mut world = World::new();

        world.register::<Position>();
        world.register::<Velocity>();

        let player = world
            .spawn_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .with(Velocity { x: 1.0, y: 1.0 })
            .id();

        let position = world.get::<Position>(player).unwrap();
        let velocity = world.get::<Velocity>(player).unwrap();

        assert_eq!(position, &Position { x: 0.0, y: 0.0 });
        assert_eq!(velocity, &Velocity { x: 1.0, y: 1.0 });
    }
}
