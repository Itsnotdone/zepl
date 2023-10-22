use std::sync::mpsc::Receiver;

use glfw::{Context, Glfw, Window, WindowEvent, WindowMode};

use crate::{App, InputEvent};

pub struct Runner {
    glfw: Glfw,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Runner {
    pub fn new() -> Self {
        let mut glfw = glfw::init_no_callbacks().expect("Failed to initialize glfw");
        let (mut window, events) = glfw
            .create_window(800, 400, "Zepl App", WindowMode::Windowed)
            .expect("Failed to initialize window");

        window.make_current();
        window.set_key_polling(true);

        Self {
            glfw,
            window,
            events,
        }
    }

    pub fn handle_events(&self, app: &mut App, event: WindowEvent) {
        match event {
            WindowEvent::Key(key, _, action, _) => {
                app.get_mut_resources()
                    .get_mut::<InputEvent>()
                    .unwrap()
                    .update(key, action);
            }

            _ => {}
        }
    }

    pub fn run(mut self, mut app: App) {
        while !self.window.should_close() {
            self.window.swap_buffers();
            self.glfw.poll_events();

            glfw::flush_messages(&self.events)
                .into_iter()
                .for_each(|(_, event)| {
                    self.handle_events(&mut app, event);
                });
            app.update();
            app.get_mut_resources()
                .get_mut::<InputEvent>()
                .unwrap()
                .clear();
        }
    }
}
