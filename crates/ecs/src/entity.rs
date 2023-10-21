use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{Component, Scene};

pub struct Entity {
    pub scene: Option<Rc<RefCell<Scene>>>,
    pub components: Vec<Box<dyn Component>>,
    pub services: Vec<String>,
    pub path: Option<String>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            scene: None,
            components: Vec::new(),
            services: Vec::new(),
            path: None,
        }
    }

    pub fn link_scene(&mut self, scene: Rc<RefCell<Scene>>) {
        self.scene = Some(scene);
    }

    pub fn with(&mut self, component: impl Component + 'static) -> &mut Entity {
        self.components.push(Box::new(component));
        self
    }

    pub fn with_boxed(&mut self, component: Box<dyn Component>) -> &mut Entity {
        self.components.push(component);
        self
    }

    pub fn with_service(&mut self, service: &str) -> &mut Entity {
        self.services.push(service.to_string());
        self
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

    pub fn add_boxed(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }
    pub fn has<T: 'static>(&self) -> bool {
        for component in &self.components {
            if (component as &dyn Any).is::<T>() {
                return true;
            }
        }

        false
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.components
            .iter()
            .find_map(|component| component.as_ref().as_any().downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components
            .iter_mut()
            .find_map(|component| component.as_mut().as_mut_any().downcast_mut::<T>())
    }

    pub fn remove<T: 'static>(&mut self) {
        let position = self
            .components
            .iter()
            .position(|component| component.as_any().is::<T>());

        if let Some(position) = position {
            self.components.remove(position);
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = Some(path);
    }
}
