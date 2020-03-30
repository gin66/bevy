use crate::{
    app::{plugin::AppPlugin, App, AppBuilder},
    core::Event,
};

use super::{Window, WindowResize};
use winit::{
    event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Default)]
pub struct WinitPlugin;

impl AppPlugin for WinitPlugin {
    fn build(&self, app: AppBuilder) -> AppBuilder {
        app.set_runner(winit_runner)
    }

    fn name(&self) -> &'static str {
        "Winit"
    }
}

pub fn winit_runner(mut app: App) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let winit_window = {
        let window = app.resources.get::<Window>().unwrap();
        let winit_window = winit::window::Window::new(&event_loop).unwrap();
        winit_window.set_title(&window.title);
        winit_window.set_inner_size(winit::dpi::PhysicalSize::new(window.width, window.height));
        winit_window
    };

    app.resources.insert(winit_window);

    log::debug!("Entering render loop");
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        match event {
            event::Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                let mut window = app.resources.get_mut::<Window>().unwrap();
                window.width = size.width;
                window.height = size.height;

                let mut resize_event = app.resources.get_mut::<Event<WindowResize>>().unwrap();
                resize_event.raise(WindowResize {
                    id: window.id,
                    height: window.height,
                    width: window.width,
                });
            }
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            event::Event::MainEventsCleared => {
                app.update();
            }
            _ => (),
        }
    });
}
