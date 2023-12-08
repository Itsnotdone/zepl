use crate::{Resources, World};

pub struct Schedule {
    runners: Vec<Box<dyn Runner>>,
}
impl Schedule {
    pub fn new() -> Self {
        Self {
            runners: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: impl Runner + 'static) {
        self.runners.push(Box::new(system))
    }

    pub fn run(&mut self, world: &mut World, resources: &mut Resources) {
        self.runners
            .iter()
            .for_each(|runner| runner.run((world, resources)))
    }
}

pub trait Runner {
    fn run(&self, (_world, _resources): (&mut World, &mut Resources)) {}
}

impl<F> Runner for F
where
    F: Fn((&mut World, &mut Resources)),
{
    fn run(&self, (world, resources): (&mut World, &mut Resources)) {
        self((world, resources))
    }
}
