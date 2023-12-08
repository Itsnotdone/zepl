use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_yaml::Value;
use std::{borrow::BorrowMut, collections::HashMap};

use ecs::{Component, Entity, World};

use crate::ComponentRegistry;
pub trait WorldSerialization {
    fn serialize(any: &dyn Component) -> Value;
    fn deserialize(value: &Value) -> Box<dyn Component>;
}

impl<T: Serialize + DeserializeOwned + 'static> WorldSerialization for T {
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
pub struct SWorld {
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
}

pub struct WorldDeserializer;

impl WorldDeserializer {
    pub fn deserialize_from_source(source: &str, registry: &ComponentRegistry) -> World {
        let mut scene = World::new();

        let sscene = serde_yaml::from_str::<SWorld>(source).unwrap();

        for entry in &sscene.entries {
            let source = std::fs::read_to_string(entry.path.clone()).unwrap();
            let mut entity = EntityDeserializer::deserialize_from_source(
                entry.name.clone(),
                source.as_str(),
                registry,
            );
            scene.add_entity(entity);
        }

        scene
    }
}

pub struct WorldSerializer;

impl WorldSerializer {
    pub fn serialize_from_world(world: &World) -> SWorld {
        let mut entities = Vec::new();

        for entity in world.iter() {
            let sentity_entry = SEntityEntry {
                path: entity.path.clone(),
                name: entity.name.clone(),
            };

            entities.push(sentity_entry);
        }

        SWorld { entries: entities }
    }
}

pub struct EntitySerializer;

impl EntitySerializer {
    pub fn serialize_from_entity(entity: &Entity, registry: &ComponentRegistry) -> SEntity {
        let mut components = HashMap::new();

        for component in entity.iterate_components() {
            components.insert(
                registry.ids.get(&component.type_id()).unwrap().into(),
                registry.serialize(&**component),
            );
        }

        let sentity = SEntity { components };

        sentity
    }
}

pub struct EntityDeserializer;

impl EntityDeserializer {
    pub fn deserialize_from_source(
        entity_name: String,
        source: &str,
        registry: &ComponentRegistry,
    ) -> Entity {
        let sentity = serde_yaml::from_str::<SEntity>(source).unwrap();
        let mut entity = Entity::new();
        entity.name = entity_name;

        for (id, value) in &sentity.components {
            entity.add_boxed(registry.deserialize(id.into(), value));
        }

        entity
    }
}
