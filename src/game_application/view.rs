use glium::{self, Surface};


extern crate glutin;


pub struct View {
    display: glium::Display
}


impl View {
    pub fn new(events_loop: &glutin::EventsLoop) -> View  {
        let window = glutin::WindowBuilder::new().with_decorations(false).with_fullscreen(Some(events_loop.get_primary_monitor()));
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        View{
            display: display    
        }
    }

    pub fn draw(&self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.finish().unwrap();
    }
}
