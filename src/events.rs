use winit::event::{Event, VirtualKeyCode, WindowEvent, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct EventManager {
    event_loop: EventLoop<()>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
        }
    }

    pub fn run(self, mut callback: impl FnMut(Event<()>) -> ControlFlow + 'static) { // Cambiado a self y agregado 'static
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = callback(event);
        });
    }
}

pub enum EventType {
    KeyPressed(VirtualKeyCode),
    KeyReleased(VirtualKeyCode),
}

pub fn handle_events(event: Event<()>) -> Option<EventType> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state == ElementState::Pressed {
                    Some(EventType::KeyPressed(input.virtual_keycode?))
                } else {
                    Some(EventType::KeyReleased(input.virtual_keycode?))
                }
            }
            _ => None,
        },
        _ => None,
    }
}
