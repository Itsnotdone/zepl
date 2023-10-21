use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::HashMap;

use ecs::{Component, Entity, Scene};

use crate::ComponentRegistry;
pub trait SceneSerialization {
    fn serialize(any: &dyn Component) -> Value;
    fn deserialize(value: &Value) -> Box<dyn Component>;
}

impl<T: Serialize + DeserializeOwned + 'static> SceneSerialization for T {
    fn serialize(component: &dyn Component) -> Value {
        serde_yaml::to_value(component.as_any().downcast_ref::<T>().unwrap()).unwrap()
    }

    fn deserialize(value: &Value) -> Box<dyn Component> {
        let my_transform: Box<dyn Component> =
            Box::new(serde_yaml::from_value::<T>(value.clone()).unwrap());
        my_transform
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SScene {
    pub entries: Vec<SEntityEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SEntityEntry {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SEntity {
    #[serde(flatten)]
    pub components: HashMap<String, Value>,
    pub services: Vec<String>,
}

pub struct SceneDeserializer;

impl SceneDeserializer {
    pub fn deserialize_from_source(source: &str, registry: &ComponentRegistry) -> Scene {
        let mut scene = Scene::new();

        let sscene = serde_yaml::from_str::<SScene>(source).unwrap();

        for entry in &sscene.entries {
            let source = std::fs::read_to_string(entry.path.clone()).unwrap();
            let entity = EntityDeserializer::deserialize_from_source(source.as_str(), registry);

            scene.add_entity(&entry.name.clone(), entity);
        }

        scene
    }
}

pub struct SceneSerializer;

impl SceneSerializer {
    pub fn serialize_from_world(world: &Scene) -> SScene {
        let mut entities = Vec::new();

        for (name, entity) in &world.entities {
            if let Some(path) = &entity.path {
                let sentity_entry = SEntityEntry {
                    path: path.clone(),
                    name: name.clone(),
                };

                entities.push(sentity_entry);
            }
        }

        SScene { entries: entities }
    }
}

pub struct EntitySerializer;

impl EntitySerializer {
    pub fn serialize_from_entity(entity: &Entity, registry: &ComponentRegistry) -> SEntity {
        let mut components = HashMap::new();

        for component in &entity.components {
            components.insert(component.type_name(), registry.serialize(&**component));
        }

        let sentity = SEntity {
            components,
            services: entity.services.clone(),
        };

        sentity
    }
}

pub struct EntityDeserializer;

impl EntityDeserializer {
    pub fn deserialize_from_source(source: &str, registry: &ComponentRegistry) -> Entity {
        let sentity = serde_yaml::from_str::<SEntity>(source).unwrap();
        let mut entity = Entity::new();

        for (id, value) in &sentity.components {
            entity.add_boxed(registry.deserialize(id.into(), value));
        }

        entity.services = sentity.services;

        entity
    }
}
