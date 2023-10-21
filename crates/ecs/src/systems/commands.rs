use crate::{Scene, System, SystemType};

pub struct EntityCommands {
    pub commands: Vec<EntityCommand>,
}

impl EntityCommands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system_type: SystemType, system: impl System + 'static) {
        self.commands
            .push(EntityCommand::AddSystem(system_type, Box::new(system)));
    }
}

pub enum EntityCommand {
    AddSystem(SystemType, Box<dyn System>),
}

pub struct Commands {
    pub commands: Vec<Command>,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn change_scene(&mut self, scene: Scene) {
        self.commands.push(Command::ChangeScene(scene));
    }
}

pub enum Command {
    ChangeScene(Scene),
}
