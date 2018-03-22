use glium;
use glium::Surface;

extern crate glutin;

use std::fs::File;
use std::io::prelude::*;

mod camera;
use self::camera::CameraState;
mod shape_builder;
use self::shape_builder::{ShapeBuilder, Vertex};


pub struct GameApplication {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    shape: glium::vertex::VertexBuffer<Vertex>,
    program: glium::Program
}


fn init_shaders(display: &glium::Display, shaders_directory: &str) -> glium::Program {
    let mut vertex_shader_file = File::open(shaders_directory.to_string() + "/vertex_shader").unwrap();
    let mut fragment_shader_file = File::open(shaders_directory.to_string() + "/fragment_shader").unwrap();
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    vertex_shader_file.read_to_string(&mut vertex_shader_src).expect("Failed to read vertex shader file!");
    fragment_shader_file.read_to_string(&mut fragment_shader_src).expect("Failed to read fragment shader file!");
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}


impl GameApplication {
    pub fn new(shaders_directory: &str) -> GameApplication {
        // glium and glutin init
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_decorations(false).with_fullscreen(Some(events_loop.get_primary_monitor()));
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // shape init
        let shape = ShapeBuilder::new(&display);

        // shaders init
        let program = init_shaders(&display, &shaders_directory);

        GameApplication{
            events_loop: events_loop,
            display: display,
            shape: shape,
            program: program
        }
    }


    pub fn start_loop(&mut self) {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let mut camera = CameraState::new(aspect_ratio, (0.0, 0.0, 0.0), (0.0, 0.0, 1.0));

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
            .. Default::default()
        };

        let mut closed = false;
        while !closed {
            let model = [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 2.0, 1.0f32]
            ];

            camera.update();
            let view = camera.get_view();
            let perspective = camera.get_perspective();
            let light = [1.4, 0.4, -0.7f32];

            let mut target = self.display.draw();
            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            target.draw(&self.shape, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &self.program,
                        &uniform!{model: model, view: view, perspective: perspective, u_light: light},
                        &params).unwrap();
            target.finish().unwrap();

            self.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent{event, ..} = event {
                    camera.process_input(&event);
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
