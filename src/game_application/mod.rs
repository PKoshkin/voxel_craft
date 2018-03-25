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
mod camera;


pub struct GameApplication<'time> {
    directory: &'time str
}


impl<'time> GameApplication<'time> {
    pub fn new(directory: &'time str) -> GameApplication<'time> {
        GameApplication{
            directory: directory
        }
    }

    pub fn start_loop(&mut self) {
        let mut closed = false;

        let mut events_loop = glutin::EventsLoop::new();
        let mut view = View::new(&events_loop, self.directory);
        let mut model = Model::new();
        let controller = Controller::new(&model);

        while !closed {
            events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent{event, ..} = event {
                    controller.handle_event(&event);
                    view.camera.handle_event(&event);
                    if let glutin::WindowEvent::KeyboardInput{input, ..} = event {
                        if let Some(key) = input.virtual_keycode {
                            if let glutin::VirtualKeyCode::Escape = key {
                                closed = true;
                            }
                        }
                    }
                }
            });
            view.camera.update();
            let draw_params = model.get_draw_params();
            view.draw(draw_params);
        }
    }
}
