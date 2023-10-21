use crate::{Commands, Entity, Resources};

pub trait System {
    fn run(&self, entity: &mut Entity, commands: &mut Commands, resources: &mut Resources);
}

impl<F> System for F
where
    F: Fn(&mut Entity, &mut Commands, &mut Resources),
{
    fn run(&self, entity: &mut Entity, commands: &mut Commands, resources: &mut Resources) {
        self(entity, commands, resources);
    }
}
