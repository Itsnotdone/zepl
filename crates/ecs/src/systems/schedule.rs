use crate::{Commands, Entity, Resources, System};

pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn run(
        &mut self,
        entity: &mut Entity,
        resources: &mut Resources,
        commands: &mut Commands,
    ) {
        self.systems.iter().for_each(|system| system.run(entity, commands, resources));
    }
}

pub enum SystemType {
    Update,
    Setup,
}
