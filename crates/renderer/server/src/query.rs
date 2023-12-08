use crate::{RenderFrame, RenderObject, RenderingServer};

pub struct RenderQuery {
    objects: Vec<RenderObject>,
}

impl RenderQuery {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn push(&mut self, object: RenderObject) {
        self.objects.push(object);
    }

    pub fn append(&mut self, objects: &mut Vec<RenderObject>) {
        self.objects.append(objects);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn draw<'a>(&'a self, server: &'a RenderingServer, mut frame: RenderFrame<'a>) {
        self.objects
            .iter()
            .for_each(|object| object.draw(server, &mut frame));
    }
}
