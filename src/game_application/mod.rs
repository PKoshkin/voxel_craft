extern crate glutin;
extern crate cgmath;

mod view;
use self::view::View;

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

    fn handle_close_event(&self, event: &glutin::WindowEvent) -> bool {
        if let glutin::WindowEvent::KeyboardInput{input, ..} = *event {
            if let Some(key) = input.virtual_keycode {
                if let glutin::VirtualKeyCode::Escape = key {
                    return true;
                }
            }
        }
        false
    }

    pub fn start_loop(&mut self) {
        let camera_position = (0.0, 0.0, 0.0);
        let mut events_loop = glutin::EventsLoop::new();
        let mut view = View::new(&events_loop, self.directory, camera_position);
        let mut model = Model::new(camera_position);

        let mut closed = false;
        while !closed {
            events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent{event, ..} = event {
                    view.camera.handle_event(&event);
                    closed = self.handle_close_event(&event);
                }
            });
            view.camera.update();
            let draw_params = model.get_draw_params(view.camera.position);
            view.draw(draw_params);
        }
    }
}
