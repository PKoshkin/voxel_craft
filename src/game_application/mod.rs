use glium;
use glium::Surface;

extern crate glutin;

extern crate image;
use std::io::Cursor;

use std::fs::File;
use std::io::prelude::*;


mod camera;
use self::camera::CameraState;
mod shape_builder;
use self::shape_builder::{ShapeBuilder, Vertex, Normal, TexCoords};


pub struct GameApplication {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    vertices: glium::VertexBuffer<Vertex>,
    normals: glium::VertexBuffer<Normal>,
    tex_coords: glium::VertexBuffer<TexCoords>,
    indices: glium::IndexBuffer<u16>,
    texture: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d,
    program: glium::Program
}


fn init_shaders(display: &glium::Display, directory: &str) -> glium::Program {
    let mut vertex_shader_file = File::open(directory.to_string() + "/shaders/vertex_shader").unwrap();
    let mut fragment_shader_file = File::open(directory.to_string() + "/shaders/fragment_shader").unwrap();
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    vertex_shader_file.read_to_string(&mut vertex_shader_src).expect("Failed to read vertex shader file!");
    fragment_shader_file.read_to_string(&mut fragment_shader_src).expect("Failed to read fragment shader file!");
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}


impl GameApplication {
    pub fn new(directory: &str) -> GameApplication {
        // glium and glutin init
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new().with_decorations(false).with_fullscreen(Some(events_loop.get_primary_monitor()));
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // shape init
        let (vertices, normals, tex_coords, indices) = ShapeBuilder::new();
        let vertices = glium::vertex::VertexBuffer::new(&display, &vertices).unwrap();
        let normals = glium::vertex::VertexBuffer::new(&display, &normals).unwrap();
        let tex_coords = glium::vertex::VertexBuffer::new(&display, &tex_coords).unwrap();
        let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        let image = image::load(Cursor::new(&include_bytes!("../../images/wood_texture.png")[..]), image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();


        let image = image::load(Cursor::new(&include_bytes!("../../images/wood_normal_map.png")[..]), image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let normal_map = glium::texture::Texture2d::new(&display, image).unwrap();

        // shaders init
        let program = init_shaders(&display, &directory);

        GameApplication{
            events_loop: events_loop,
            display: display,
            vertices: vertices,
            normals: normals,
            tex_coords: tex_coords,
            indices: indices,
            texture: texture,
            normal_map: normal_map,
            program: program
        }
    }

    pub fn start_loop(&mut self) {
        let (width, height) = self.display.get_framebuffer_dimensions();
        let aspect_ratio = width as f32 / height as f32;
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
            target.draw((&self.vertices, &self.normals, &self.tex_coords),
                        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &self.program,
                        &uniform!{model: model, view: view, perspective: perspective, diffuse_tex: &self.texture, normal_tex: &self.normal_map, u_light: light},
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
