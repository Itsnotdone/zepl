use crate::World;

pub struct Commands {
    pub commands: Vec<Command>,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn change_scene(&mut self, scene: World) {
        self.commands.push(Command::ChangeScene(scene));
    }
}

pub enum Command {
    ChangeScene(World),
}
