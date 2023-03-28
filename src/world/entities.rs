use std::any::TypeId;

use super::{
    components::{Component, ComponentStorage, DenseVecStorage},
    World,
};

pub type EntityId = u32;

pub struct EntityManager {
    next_id: EntityId,
    deleted_entities: Vec<EntityId>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            deleted_entities: vec![],
        }
    }

    /// Returns the next available entity id.
    pub fn next_id(&mut self) -> EntityId {
        if let Some(id) = self.deleted_entities.pop() {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }

    pub fn delete(&mut self, entity: EntityId) {
        self.deleted_entities.push(entity);
    }
}

pub struct EntityBuilder<'a> {
    entity_id: EntityId,
    world: &'a mut World,
}

impl<'a> EntityBuilder<'a> {
    pub fn new(world: &'a mut World) -> Self {
        let entity_id = world.entity_manager.next_id();
        world.entity_manager.next_id += 1;

        Self {
            entity_id,
            world,
        }
    }

    pub fn with<C: Component + 'static>(&mut self, component: C) -> &mut Self {
        let entity = self.entity_id;

        self.world
            .component_manager
            .insert_component(entity, component);

        self
    }

    pub fn id(&self) -> EntityId {
        self.entity_id
    }
}

//-- Tests --//

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;

    impl Component for u32 {
        type Storage = DenseVecStorage<Self>;
    }

    #[test]
    fn entity_builder() {
        let mut world = World::new();

        world.register::<u32>();

        let mut first_builder = EntityBuilder::new(&mut world);
        first_builder.with(1u32);
        assert!(first_builder.id() == 0);

        drop(first_builder);

        let mut second_builder = EntityBuilder::new(&mut world);
        second_builder.with(2u32);
        dbg!(second_builder.id());
        //assert!(second_builder.id() == 1);
    }
}
