use game_application::vertex::Vertex;


pub struct Map {

}


impl Map {
    pub fn new() -> Map {
        Map{}
    }

    pub fn update(&self) {

    }

    pub fn add_vertices(&self, shape: &mut Vec<Vertex>, indices: &mut Vec<u16>) {
        shape.extend_from_slice(&[
            Vertex{position: [-1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 1.0]},
            Vertex{position: [ 1.0,  1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 1.0]},
            Vertex{position: [-1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [0.0, 0.0]},
            Vertex{position: [ 1.0, -1.0, 0.0], normal: [0.0, 0.0, -1.0], tex_coords: [1.0, 0.0]}
        ]);
        indices.extend_from_slice(&[
            0, 1, 2,
            1, 2, 3
        ]);
    }
}
