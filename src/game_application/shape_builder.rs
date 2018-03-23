#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, normal, tex_coords);


pub struct ShapeBuilder;


impl ShapeBuilder {
    pub fn new() -> (Vec<Vertex>, Vec<u16>) {
        let shape = vec![
            Vertex{position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0]},
            Vertex{position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0]},
            Vertex{position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0]},
            Vertex{position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0]},
        ];
        let indices = vec![
            0, 1, 2,
            1, 2, 3
        ];
        (shape, indices)
    }
}
