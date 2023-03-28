use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use super::entities::EntityId;

pub trait Component: Sized {
    type Storage: ComponentStorage<Self>;
}

pub trait ComponentStorage<C: Component> {
    fn new() -> Self;
    fn insert(&mut self, entity: EntityId, component: C);
    fn remove(&mut self, entity: EntityId);
    fn get(&self, entity: EntityId) -> Option<&C>;
    fn get_mut(&mut self, entity: EntityId) -> Option<&mut C>;
}

pub struct DenseVecStorage<C: Component> {
    components: Vec<(EntityId, C)>,
}

impl<C: Component> ComponentStorage<C> for DenseVecStorage<C> {
    fn new() -> Self {
        Self { components: vec![] }
    }

    fn insert(&mut self, entity: EntityId, component: C) {
        self.components.push((entity, component));
    }

    fn remove(&mut self, entity: EntityId) {
        self.components.retain(|(e, _)| *e != entity);
    }

    fn get(&self, entity: EntityId) -> Option<&C> {
        self.components
            .iter()
            .find(|(e, _)| *e == entity)
            .map(|(_, c)| c)
    }

    fn get_mut(&mut self, entity: EntityId) -> Option<&mut C> {
        self.components
            .iter_mut()
            .find(|(e, _)| *e == entity)
            .map(|(_, c)| c)
    }
}

pub struct ComponentManager {
    pub components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register<C: Component + 'static, S: ComponentStorage<C> + 'static>(&mut self) {
        self.components
            .insert(TypeId::of::<C>(), Box::new(S::new()));
    }

    pub fn get<C: Component + 'static>(&self, entity: EntityId) -> Option<&C> {
        let storage = self
            .components
            .get(&TypeId::of::<C>())
            .and_then(|c| c.downcast_ref::<DenseVecStorage<C>>())
            .unwrap();

        storage.get(entity)
    }

    pub fn get_mut<C: Component + 'static>(&mut self, entity: EntityId) -> Option<&mut C> {
        let storage = self
            .components
            .get_mut(&TypeId::of::<C>())
            .and_then(|c| c.downcast_mut::<DenseVecStorage<C>>())
            .unwrap();

        storage.get_mut(entity)
    }

    pub fn insert_component<C: Component + 'static>(&mut self, entity: EntityId, component: C) {
        let storage = self
            .components
            .get_mut(&TypeId::of::<C>())
            .and_then(|c| c.downcast_mut::<DenseVecStorage<C>>())
            .unwrap();

        storage.insert(entity, component);
    }
}

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
    fn component_manager() {
        let mut component_manager = ComponentManager::new();
        component_manager.register::<Position, DenseVecStorage<Position>>();
        component_manager.register::<Velocity, DenseVecStorage<Velocity>>();

        component_manager.insert_component(0, Position { x: 0.0, y: 0.0 });
        component_manager.insert_component(0, Velocity { x: 1.0, y: 1.0 });

        let position = component_manager.get::<Position>(0).unwrap();
        assert_eq!(position.x, 0.0);
        assert_eq!(position.y, 0.0);

        let velocity = component_manager.get::<Velocity>(0).unwrap();
        assert_eq!(velocity.x, 1.0);
        assert_eq!(velocity.y, 1.0);
    }
}
