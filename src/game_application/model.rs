use game_application::draw_params::{DrawParams, UniformsStruct};
use game_application::map::Map;


pub struct Model {
    map: Map
}


impl Model {
    pub fn new() -> Model {
        let map = Map::new(0.1, (10, 10, 10));
        Model{
            map: map
        }
    }

    pub fn update(&mut self) {
        self.map.build_voxels();
    }

    pub fn get_draw_params(&self) -> DrawParams {
        let mut shape = Vec::new();
        let mut indices = Vec::new();
        self.map.add_vertices(&mut shape, &mut indices);
        DrawParams{
            shape: shape,
            indices: indices,
            uniforms: UniformsStruct{}
        }
    }
}
