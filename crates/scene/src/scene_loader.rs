use crate::ComponentRegistry;

pub struct SceneLoader {
    registry: ComponentRegistry,
}

impl SceneLoader {
    pub fn new(registry: ComponentRegistry) -> Self {
        Self { registry: registry }
    }

    pub fn get_registry(&self) -> &ComponentRegistry {
        &self.registry
    }
}
