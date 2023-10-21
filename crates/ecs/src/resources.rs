use std::any::Any;

pub struct Resources {
    resources: Vec<Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource: impl Any + 'static) {
        self.resources.push(Box::new(resource));
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
}
