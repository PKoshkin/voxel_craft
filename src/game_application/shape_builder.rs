use glium;


#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);


pub struct ShapeBuilder;


impl ShapeBuilder {
    pub fn new(display: &glium::Display) -> glium::vertex::VertexBuffer<Vertex> {
        glium::vertex::VertexBuffer::new(display, &[
            Vertex{position: [-1.0,  1.0, 1.0], normal: [0.0, 0.0, -1.0]},
            Vertex{position: [ 1.0,  1.0, 1.0], normal: [0.0, 0.0, -1.0]},
            Vertex{position: [-1.0, -1.0, 1.0], normal: [0.0, 0.0, -1.0]},
        ]).unwrap()
    }
}
