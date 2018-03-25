use game_application::glutin;
use game_application::model::Model;


pub struct Controller<'time> {
    model: &'time Model
}


impl<'time> Controller<'time> {
    pub fn new(model: &'time Model) -> Controller<'time> {
        Controller{
            model: model
        }
    }

    pub fn handle_event(&self, event: &glutin::WindowEvent) {
        
    }
}
