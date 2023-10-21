use serde_yaml::Value;
use std::collections::HashMap;

use ecs::Component;

use crate::SceneSerialization;

pub struct ComponentRegistry {
    pub components: HashMap<String, ComponentRegistration>,
}

pub struct ComponentRegistration {
    deserializer: Box<dyn Fn(&Value) -> Box<dyn Component>>,
    serializer: Box<dyn Fn(&dyn Component) -> Value>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register<T: SceneSerialization + 'static>(&mut self, id: &str) {
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
        let registration = self.components.get(&component.type_name()).unwrap();

        (registration.serializer)(component)
    }
}
