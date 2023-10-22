use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use ecs::{
    Command, Commands, Entity, EntityCommand, EntityCommands, Resources, Scene, ScheduleGraph,
};

use scene::{ComponentRegistry, SceneLoader, Transform2d};

use crate::{Config, Game, InputEvent, Runner};
pub struct App {
    scene: Rc<RefCell<Scene>>,
    schedule_graph: ScheduleGraph,
    resources: Resources,
    config: Config,
    game: Game,
}

impl App {
    pub fn new(config: &str) -> Self {
        let config = serde_yaml::from_str::<Config>(config).unwrap();
        let game = Game::new(config.runtime.dylib.as_str());
        let mut resources = Resources::new();
        let mut registry = ComponentRegistry::new();

        registry.register::<Transform2d>("Transform2d");

        resources.add_resource(SceneLoader::new(registry));

        Self {
            scene: Rc::new(RefCell::new(Scene::new())),
            schedule_graph: ScheduleGraph::new(),
            resources: resources,
            config: config,
            game: game,
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        let scene = Rc::new(RefCell::new(scene));

        scene
            .borrow_mut()
            .entities
            .iter_mut()
            .for_each(|(name, entity)| {
                entity.services.iter().for_each(|service| {
                    let mut commands = EntityCommands::new();

                    self.game.call_service(service.as_str(), &mut commands);

                    self.execute_entity_commands(name.into(), commands);
                });
                entity.link_scene(scene.clone());
            });

        self.scene = scene;
    }

    pub fn get_mut(&mut self, name: String) -> Option<RefMut<Entity>> {
        let result = RefMut::filter_map(self.scene.borrow_mut(), |scene| {
            scene.get_mut(name.as_str())
        });

        if let Ok(entity) = result {
            return Some(entity);
        }

        None
    }

    pub fn get_resources(&mut self) -> &Resources {
        &self.resources
    }

    pub fn get_mut_resources(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn execute_entity_commands(&mut self, name: String, commands: EntityCommands) {
        for command in commands.commands {
            match command {
                EntityCommand::AddSystem(system_type, system) => {
                    self.schedule_graph
                        .add_system(name.clone(), system_type, system)
                }
            }
        }
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
        self.scene
            .borrow_mut()
            .entities
            .iter_mut()
            .for_each(|(name, entity)| {
                self.schedule_graph
                    .run(name.clone(), entity, &mut commands, &mut self.resources);
            });
        self.execute_commands(commands);
    }

    pub fn run(mut self) {
        let mut commands = Commands::new();
        self.game.call_main(
            self.config.runtime.main.as_str(),
            &mut commands,
            &mut self.resources,
        );
        self.execute_commands(commands);
        self.resources.add_resource(InputEvent::default());

        let runner = Runner::new();

        runner.run(self);
    }
}
