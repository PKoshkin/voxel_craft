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

#[derive(Copy, Clone)]
pub struct TexCoords {
    tex_coords: [f32; 2],
}
implement_vertex!(TexCoords, tex_coords);


pub struct ShapeBuilder;


impl ShapeBuilder {
    pub fn new() -> (Vec<Vertex>, Vec<Normal>, Vec<TexCoords>, Vec<u16>) {
        let vertices = vec![
            Vertex{position: [-1.0,  1.0, 1.0]},
            Vertex{position: [ 1.0,  1.0, 1.0]},
            Vertex{position: [-1.0, -1.0, 1.0]},
            Vertex{position: [ 1.0, -1.0, 1.0]}
        ];
        let normals = vec![
            Normal{normal: [0.0, 0.0, -1.0]},
            Normal{normal: [0.0, 0.0, -1.0]},
            Normal{normal: [0.0, 0.0, -1.0]},
            Normal{normal: [0.0, 0.0, -1.0]}
        ];
        let tex_coords = vec![
            TexCoords{tex_coords: [0.0, 1.0]},
            TexCoords{tex_coords: [1.0, 1.0]},
            TexCoords{tex_coords: [0.0, 0.0]},
            TexCoords{tex_coords: [1.0, 0.0]},
        ];
        let indices = vec![
            0, 1, 2, 3
        ];

        (vertices, normals, tex_coords, indices)
    }
}
