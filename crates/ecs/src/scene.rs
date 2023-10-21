use hashbrown::HashMap;

use crate::Entity;

pub struct Scene {
    pub entities: HashMap<String, Entity>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn with_entity(mut self, name: &str, entity: Entity) -> Scene {
        self.entities.insert(name.to_string().clone(), entity);
        self
    }

    pub fn add_entity(&mut self, name: &str, entity: Entity) -> &mut Entity {
        self.entities.insert(name.to_string().clone(), entity);
        self.entities.get_mut(&name.to_string()).unwrap()
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Entity> {
        self.entities.get_mut(&name.to_string())
    }
}
