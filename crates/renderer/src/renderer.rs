use ecs::{Entity, Resources, World};
use glfw::Window;
use std::{cell::RefCell, rc::Rc};
use wgpu::SurfaceError;

use server::RenderingServer;

use crate::Draw;

pub struct Renderer {
    server: RenderingServer,
    window_size: (i32, i32),
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let server = pollster::block_on(RenderingServer::new(window));
        let window_size = window.get_size();
        Self {
            server,
            window_size,
        }
    }

    // pub fn added_entity(&mut self, entity: &Entity) {
    //     self.server
    //         .append(&mut entity.into_render_objects(&self.server));
    // }

    pub fn update(&mut self, resources: &mut Resources, world: &mut World) -> NextAction {
        let mut render_objects = Vec::new();

        world.iter_mut().for_each(|entity| {
            if let Some(object) = entity.draw(&mut self.server, resources){
                render_objects.push(object);
            }
            
        });


        self.server.append(&mut render_objects);
        match self.server.draw() {
            Ok(_) => return NextAction::Nothing,

            // lost or outdated
            Err(SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                return NextAction::ReconfigureWindow
            }
            // The system is out of memory,
            Err(SurfaceError::OutOfMemory) => return NextAction::Exit,

            // ignoring timeouts
            Err(SurfaceError::Timeout) => return NextAction::IgnoreSurfaceTimeout,
        }
    }

    pub fn reconfigure(&mut self, new_size: (i32, i32)) {
        self.server.reconfigure(new_size);
    }
}

pub enum NextAction {
    Nothing,
    ReconfigureWindow,
    IgnoreSurfaceTimeout,
    Exit,
}
