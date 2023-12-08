use ecs::{Command, Commands, Entity, Resources, ScheduleGraph, World};

use renderer::Renderer;
use scene::{ComponentRegistry, SceneLoader, Transform2d};

use crate::{Config, Game, InputEvent, Runner};
pub struct App {
    pub(crate) scene: World,
    pub(crate) resources: Resources,

    schedule_graph: ScheduleGraph,
    config: Config,
    system_list: SystemList,
    game: Game,
}

impl App {
    pub fn new(config: &str, systems: &'static str) -> Self {
        let config = serde_yaml::from_str::<Config>(config).unwrap();
        let game = Game::new(config.runtime.dylib.as_str());
        let mut resources = Resources::new();
        let mut registry = ComponentRegistry::new();

        registry.register::<Transform2d>("Transform2d");

        resources.add_resource(SceneLoader::new(registry));

        Self {
            scene: World::new(),
            schedule_graph: ScheduleGraph::new(),
            resources: resources,
            config: config,
            game: game,
            system_list: SystemList::new(systems),
        }
    }

    pub fn set_scene(&mut self, scene: World) {
        self.scene = scene;
    }

    pub fn get_resources(&mut self) -> &Resources {
        &self.resources
    }

    pub fn execute_commands(&mut self, commands: Commands) {
        for command in commands.commands {
            match command {
                Command::ChangeScene(scene) => self.set_scene(scene),
            }
        }
    }

    pub fn get_scene_loader(&self) -> &SceneLoader {
        self.resources.get::<SceneLoader>().unwrap()
    }

    pub fn update(&mut self) {
        let mut commands = Commands::new();

        self.schedule_graph
            .run((&mut self.scene, &mut self.resources));

        self.execute_commands(commands);
    }

    pub fn run(mut self) {
        let mut commands = Commands::new();
        self.game.call_main(
            self.config.runtime.main.as_str(),
            &mut commands,
            &mut self.resources,
        );

        for system in &self.system_list.systems {
            self.game
                .call_system_builder(system, &mut self.schedule_graph);
        }

        self.execute_commands(commands);
        self.resources.add_resource(InputEvent::default());

        let runner = Runner::new();
        let renderer = Renderer::new(runner.get_window());

        runner.run(self, renderer);
    }
}

struct SystemList {
    pub systems: Vec<&'static str>,
}

impl SystemList {
    pub fn new(data: &'static str) -> Self {
        let systems = data.trim().split('\n').collect();

        Self { systems }
    }
}
