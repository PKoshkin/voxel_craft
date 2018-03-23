#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
pub struct Normal {
    normal: [f32; 3],
}
implement_vertex!(Normal, normal);


pub struct ShapeBuilder;


impl ShapeBuilder {
    pub fn new() -> (Vec<Vertex>, Vec<Normal>, Vec<u16>) {
        let vertices = vec![
            Vertex{position: [-1.0,  1.0, 1.0]},
            Vertex{position: [ 1.0,  1.0, 1.0]},
            Vertex{position: [-1.0, -1.0, 1.0]}
        ];
        let normals = vec![
            Normal{normal: [0.0, 0.0, -1.0]},
            Normal{normal: [0.0, 0.0, -1.0]},
            Normal{normal: [0.0, 0.0, -1.0]}
        ];
        let indices = vec![
            0, 1, 2
        ];

        (vertices, normals, indices)
    }
}
