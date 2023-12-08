use std::{
    any::Any,
    slice::{Iter, IterMut},
};

use crate::Component;

pub struct Entity {
    components: Vec<Box<dyn Component>>,

    pub name: String,
    pub path: String,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            name: String::new(),
            path: String::new(),
        }
    }

    pub fn with(&mut self, component: impl Component + 'static) -> &mut Entity {
        self.components.push(Box::new(component));
        self
    }

    pub fn with_boxed(&mut self, component: Box<dyn Component>) -> &mut Entity {
        self.components.push(component);
        self
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

    pub fn add_boxed(&mut self, component: Box<dyn Component>) {
        self.components.push(component);
    }

    pub fn has<T: 'static>(&self) -> bool {
        for component in &self.components {
            if component.as_ref().as_any().is::<T>() {
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

    pub fn iterate_components(&self) -> Iter<Box<dyn Component>> {
        self.components.iter()
    }

    pub fn iterate_components_mut(&mut self) -> IterMut<Box<dyn Component>> {
        self.components.iter_mut()
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
