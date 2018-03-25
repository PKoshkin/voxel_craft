extern crate glutin;
extern crate cgmath;

mod view;
use self::view::View;

mod controller;
use self::controller::Controller;

mod model;
use self::model::Model;

mod draw_params;
mod vertex;
mod map;


pub struct GameApplication {
    events_loop: glutin::EventsLoop,
    view: View,
    model: Model,
}


impl GameApplication {
    pub fn new(directory: &str) -> GameApplication {
        let events_loop = glutin::EventsLoop::new();
        let view = View::new(&events_loop);
        let model = Model::new();
        GameApplication{
            events_loop: events_loop,
            view: view,
            model: model
        }
    }

    pub fn start_loop(&mut self) {
        let mut closed = false;
        let controller = Controller::new(&self.model);
        while !closed {
            self.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent{event, ..} = event {
                    controller.handle_event(&event);
                    if let glutin::WindowEvent::KeyboardInput{input, ..} = event {
                        if let Some(key) = input.virtual_keycode {
                            if let glutin::VirtualKeyCode::Escape = key {
                                closed = true;
                            }
                        }
                    }
                }
            });
            let draw_params = self.model.get_draw_params();
            self.view.draw(draw_params);
        }
    }
}
