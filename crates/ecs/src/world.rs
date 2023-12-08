use std::slice::{Iter, IterMut};

use crate::Entity;

pub struct World {
    entities: Vec<Entity>,
}

impl World {
    pub fn new() -> World {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }

    pub fn iter(&self) -> Iter<Entity> {
        self.entities.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Entity> {
        self.entities.iter_mut()
    }
}
