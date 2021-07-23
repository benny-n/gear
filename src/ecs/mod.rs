mod tests;
pub mod entity;
pub mod scene;
pub mod component;
pub use self::component::Component;
pub use self::entity::Entity;
pub use std::{collections::HashMap, error::Error};
pub type EntityId = i64;
pub type ComponentTypeId = i64;
pub type ComponentPool = HashMap<EntityId, Box<dyn Component>>;