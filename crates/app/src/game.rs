use ecs::{Commands, EntityCommands, Resources};
use libloading::*;

pub struct Game {
    library: Library,
}

impl Game {
    pub fn new(path: &str) -> Self {
        unsafe {
            let lib = Library::new(path).unwrap();

            return Self { library: lib };
        }
    }

    pub fn call_service(&self, service: &str, commands: &mut EntityCommands) {
        unsafe {
            let func = self
                .library
                .get::<unsafe extern "C" fn(&mut EntityCommands)>(service.as_bytes())
                .unwrap();
            func(commands)
        }
    }

    pub fn call_main(&self, main: &str, commands: &mut Commands, resources: &mut Resources) {
        unsafe {
            let func = self
                .library
                .get::<unsafe extern "C" fn(&mut Commands, &mut Resources)>(main.as_bytes())
                .unwrap();
            func(commands, resources)
        }
    }
}
