use glium::{self, Surface};
use game_application::glutin;
use game_application::draw_params::DrawParams;
use game_application::camera::Camera;

use std::fs::File;
use std::io::prelude::*;

extern crate image;
use std::io::Cursor;


pub struct View<'time> {
    pub camera: Camera,
    directory: &'time str,
    display: glium::Display,
    texture: glium::texture::SrgbTexture2d,
    normal_map: glium::texture::Texture2d
}


fn init_textured_triangles_program(display: &glium::Display, directory: &str) -> glium::Program {
    let mut vertex_shader_file = File::open(directory.to_string() + "/src/shaders/textured_triangles.vert").unwrap();
    let mut fragment_shader_file = File::open(directory.to_string() + "/src/shaders/textured_triangles.frag").unwrap();
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    vertex_shader_file.read_to_string(&mut vertex_shader_src).expect("Failed to read vertex shader file!");
    fragment_shader_file.read_to_string(&mut fragment_shader_src).expect("Failed to read fragment shader file!");
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}


fn init_points_program(display: &glium::Display, directory: &str) -> glium::Program {
    let mut vertex_shader_file = File::open(directory.to_string() + "/src/shaders/points.vert").unwrap();
    let mut fragment_shader_file = File::open(directory.to_string() + "/src/shaders/points.frag").unwrap();
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();
    vertex_shader_file.read_to_string(&mut vertex_shader_src).expect("Failed to read vertex shader file!");
    fragment_shader_file.read_to_string(&mut fragment_shader_src).expect("Failed to read fragment shader file!");
    glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap()
}


fn init_textures(display: &glium::Display) -> (glium::texture::SrgbTexture2d, glium::texture::Texture2d) {
    let image = image::load(Cursor::new(&include_bytes!("../../images/grass_texture.jpg")[..]), image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../../images/grass_normal_map.png")[..]), image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let normal_map = glium::texture::Texture2d::new(display, image).unwrap();

    (texture, normal_map)
}


impl<'time> View<'time> {
    pub fn new(events_loop: &glutin::EventsLoop, directory: &'time str) -> View<'time>  {
        let window = glutin::WindowBuilder::new().with_decorations(false).with_fullscreen(Some(events_loop.get_primary_monitor()));
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // textires init
        let (texture, normal_map) = init_textures(&display);

        // camera init
        let (width, height) = display.get_framebuffer_dimensions();
        let aspect_ratio = width as f32 / height as f32;
        let camera = Camera::new(aspect_ratio, (0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        View{
            camera: camera,
            directory: directory,
            display: display,
            texture: texture,
            normal_map: normal_map
        }
    }


    pub fn draw(&self, draw_params: DrawParams) {
        let program = init_points_program(&self.display, &self.directory);
        let shape = glium::vertex::VertexBuffer::new(&self.display, &draw_params.shape).unwrap();
        let indices = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TrianglesList, &draw_params.indices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockWise,
            .. Default::default()
        };
        let mut target = self.display.draw();
        let view = self.camera.get_view();
        let perspective = self.camera.get_perspective();
        let light = [1.4, 0.4, -0.7f32];
        let model = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 2.0, 1.0f32]
        ];
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target.draw(&shape, &indices, &program,
                    &uniform!{model: model, view: view, perspective: perspective},
                    &params).unwrap();
        target.finish().unwrap();
    }
}
