use game_application::draw_params::{DrawParams, UniformsStruct};
use game_application::map::Map;
use game_application::glutin;
use game_application::cgmath::Point3;


pub struct Model {
    map: Map
}


impl Model {
    pub fn new(camera_position: Point3<f32>) -> Model {
        // Размер карты должен представляться в виде 2 ^ i + 1
        let mut map = Map::new(0.01, 257, camera_position);
        map.build_voxels();
        Model{
            map: map
        }
    }

    pub fn get_draw_params(&mut self, camera_position: Point3<f32>) -> DrawParams {
        let shape = self.map.get_vertices(camera_position);
        DrawParams{
            shape: shape,
            uniforms: UniformsStruct{}
        }
    }

    pub fn handle_event(&mut self, event: &glutin::WindowEvent) {
        // Обрабытывает сигналы для бизнес логики
    }
}
