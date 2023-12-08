use crate::{Resources, Runner, Schedule, World};

pub use ScheduleLabel::*;

pub struct ScheduleGraph {
    update: Schedule,
    setup: Schedule,
    first: bool,
}

impl ScheduleGraph {
    pub fn new() -> Self {
        Self {
            update: Schedule::new(),
            setup: Schedule::new(),
            first: true,
        }
    }

    pub fn insert<T>(&mut self, label: ScheduleLabel, runner: T)
    where
        T: Runner + 'static,
    {
        match label {
            Update => self.update.add_system(runner),
            Setup => self.setup.add_system(runner),
        }
    }

    pub fn run(&mut self, (world, resources): (&mut World, &mut Resources)) {
        if self.first {
            self.setup.run(world, resources);
        }
        self.update.run(world, resources);

        self.first = false;
    }
}

pub enum ScheduleLabel {
    Update,
    Setup,
}
