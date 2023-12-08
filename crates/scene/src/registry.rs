use serde_yaml::Value;
use std::{any::TypeId, collections::HashMap};

use ecs::Component;

use crate::WorldSerialization;

pub struct ComponentRegistry {
    pub ids: HashMap<TypeId, String>,
    pub components: HashMap<String, ComponentRegistration>,
}

pub struct ComponentRegistration {
    deserializer: Box<dyn Fn(&Value) -> Box<dyn Component>>,
    serializer: Box<dyn Fn(&dyn Component) -> Value>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            ids: HashMap::new(),
            components: HashMap::new(),
        }
    }

    pub fn register<T: WorldSerialization + 'static>(&mut self, id: &str) {
        self.ids.insert(TypeId::of::<T>(), id.to_string());
        self.components.insert(
            id.to_string(),
            ComponentRegistration {
                deserializer: Box::new(T::deserialize),
                serializer: Box::new(T::serialize),
            },
        );
    }

    pub fn deserialize(&self, id: String, value: &Value) -> Box<dyn Component> {
        let registration = self.components.get(&id).unwrap();

        (registration.deserializer)(value)
    }

    pub fn serialize(&self, component: &dyn Component) -> Value {
        let id = self.ids.get(&component.type_id()).unwrap();
        let registration = self.components.get(id).unwrap();

        (registration.serializer)(component)
    }
}
