use game_application::draw_params::{DrawParams, UniformsStruct};
use game_application::map::Map;


pub struct Model {
    map: Map
}


impl Model {
    pub fn new(camera_position: (f32, f32, f32)) -> Model {
        let mut map = Map::new(0.1, (10, 10, 10), camera_position);
        map.build_voxels();
        Model{
            map: map
        }
    }

    pub fn get_draw_params(&mut self, camera_position: (f32, f32, f32)) -> DrawParams {
        let shape = self.map.get_vertices(camera_position);
        DrawParams{
            shape: shape,
            uniforms: UniformsStruct{}
        }
    }
}
