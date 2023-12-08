use std::any::Any;

use crate::Handle;

pub struct Resources {
    resources: Vec<Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    pub fn add_resource<T: Any + 'static>(&mut self, resource: T) -> Handle<T> {
        self.resources.push(Box::new(resource));

        Handle::<T>::new(self.resources.len() - 1)
    }

    pub fn handle<T: 'static>(&self, handle: &Handle<T>) -> Option<&T> {
        if let Some(any) = self.resources.get(handle.index) {
            return any.downcast_ref::<T>();
        }
        None
    }

    pub fn handle_mut<T: 'static>(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        if let Some(any) = self.resources.get_mut(handle.index) {
            return any.downcast_mut::<T>();
        }
        None
    }

    pub fn get_or_init<T: Default + 'static>(&mut self) -> &T {
        if !self.has::<T>() {
            self.add_resource(T::default());
        }

        self.get::<T>().unwrap()
    }

    pub fn get_mut_or_init<T: Default + 'static>(&mut self) -> &mut T {
        if !self.has::<T>() {
            self.add_resource(T::default());
        }

        self.get_mut::<T>().unwrap()
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.resources
            .iter()
            .find_map(|resource| resource.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .iter_mut()
            .find_map(|resource| resource.downcast_mut::<T>())
    }

    pub fn has<T: 'static>(&self) -> bool {
        for resource in &self.resources {
            if resource.is::<T>() {
                return true;
            }
        }

        false
    }
}
