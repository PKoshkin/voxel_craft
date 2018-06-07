extern crate glutin;
extern crate cgmath;
extern crate rand;

mod view;
use self::view::View;

mod model;
use self::model::Model;

mod draw_params;
mod vertex;
mod map;
mod camera;
mod marching_cubes;
mod diamond_square;
mod geometry;
mod mesh;

use game_application::draw_params::DrawParams;
use game_application::cgmath::Point3;


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

    fn get_draw_params(&mut self, model: &mut Model, view: &mut View) -> DrawParams {
        // Вынимает DrawParams из model и view
        model.get_draw_params(view.camera.position)
    }

    pub fn start_loop(&mut self) {
        let camera_position = Point3::new(0.0, 0.0, 0.0);
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
            let draw_params = self.get_draw_params(&mut model, &mut view);
            view.draw(draw_params);
        }
    }
}
