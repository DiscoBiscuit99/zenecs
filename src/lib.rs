#![allow(dead_code)]

///! Example:
///! ```rust
///! use tecs::prelude::*;
///!
///! fn main() {
///!    let mut world = World::new();
///!
///!    world.register::<Position>();
///!    world.register::<Velocity>();
///!    world.spawn_entity()
///!        .with(Position { x: 0.0, y: 0.0 })
///!        .with(Velocity { x: 1.0, y: 1.0 });
///! }
mod world;

pub mod prelude {
    pub use crate::world::World;
}
