use std::sync::mpsc::Receiver;

use glfw::{ClientApiHint, Context, Glfw, Window, WindowEvent, WindowHint, WindowMode};
use renderer::{NextAction, Renderer};

use crate::{App, InputEvent};
use ecs::*;
use scene::*;

pub struct Runner {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Runner {
    pub fn new() -> Self {
        let mut glfw = glfw::init_no_callbacks().expect("Failed to initialize glfw");

        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        glfw.window_hint(WindowHint::Visible(true));

        let (mut window, events) = glfw
            .create_window(400, 400, "Zepl App", WindowMode::Windowed)
            .expect("Failed to initialize window");

        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);

        Self {
            glfw,
            window,
            events,
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn handle_events(&self, app: &mut App, renderer: &mut Renderer, event: WindowEvent) {
        match event {
            WindowEvent::Size(width, height) => {
                renderer.reconfigure((width, height));
            }

            WindowEvent::Key(key, _, action, _) => {
                app.resources
                    .get_mut::<InputEvent>()
                    .unwrap()
                    .update(key, action);
            }

            _ => {}
        }
    }

    pub fn run(mut self, mut app: App, mut renderer: Renderer) {
        while !self.window.should_close() {
            let start = std::time::Instant::now();

            self.glfw.poll_events();

            glfw::flush_messages(&self.events)
                .into_iter()
                .for_each(|(_, event)| {
                    self.handle_events(&mut app, &mut renderer, event);
                });

            app.update();
            app.resources.get_mut::<InputEvent>().unwrap().clear();

            match renderer.update(&mut app.resources, &mut app.scene) {
                NextAction::Nothing => {}
                NextAction::ReconfigureWindow => renderer.reconfigure(self.window.get_size()),
                NextAction::Exit => self.window.set_should_close(true),
                NextAction::IgnoreSurfaceTimeout => log::warn!("Surface timeout"),
            }
            self.window.swap_buffers();

            let end = std::time::Instant::now();
            println!("Current FPS: {:?}", (1.0 / (end - start).as_secs_f32()));
        }
    }
}
