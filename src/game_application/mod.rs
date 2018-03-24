extern crate glutin;


mod view;
use self::view::View;


pub struct GameApplication {
    events_loop: glutin::EventsLoop,
    view: View
}


impl GameApplication {
    pub fn new(directory: &str) -> GameApplication {
        let events_loop = glutin::EventsLoop::new();
        let view= View::new(&events_loop);
        GameApplication{
            events_loop: events_loop,
            view: view
        }
    }

    pub fn start_loop(&mut self) {
        let mut closed = false;
        while !closed {
            self.view.draw();
            self.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent{event, ..} = event {
                    if let glutin::WindowEvent::KeyboardInput{input, ..} = event {
                        if let Some(key) = input.virtual_keycode {
                            if let glutin::VirtualKeyCode::Escape = key {
                                closed = true;
                            }
                        }
                    }
                }
            });
        }
    }
}
