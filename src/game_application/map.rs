use game_application::vertex::Vertex;
use game_application::marching_cubes::get_vertices;
use game_application::diamond_square::generate_hight_map;
use game_application::cgmath::Point3;


pub struct Map {
    voxel_size: f32,
    map_size: usize,
    voxels: Vec<Vec<Vec<bool>>>,
    camera_position: Point3<f32>,
    vertices: Vec<Vertex>
}


impl Map {
    pub fn new(voxel_size: f32, map_size: usize, camera_position: Point3<f32>) -> Map {
        let mut voxels = Vec::new();
        for x in 0..map_size {
            voxels.push(Vec::new());
            for y in 0..map_size {
                voxels[x].push(Vec::new());
                for _ in 0..map_size {
                    voxels[x][y].push(false);
                }
            }
        }
        Map {
            voxel_size: voxel_size,
            map_size: map_size,
            voxels: voxels,
            camera_position: camera_position,
            vertices: Vec::new()
        }
    }

    pub fn build_voxels(&mut self) {
        let high_map = generate_hight_map(self.map_size);
        for (y, vector) in high_map.iter().enumerate() {
            for (x, height) in vector.iter().enumerate() {
                for z in 0..(*height + 1) {
                    self.voxels[x][y][z] = true;
                }
            }
        }
    }

    fn need_to_redraw(&self, camera_position: Point3<f32>) -> bool {
        self.vertices.len() == 0
    }

    pub fn get_vertices(&mut self, camera_position: Point3<f32>) -> Vec<Vertex> {
        if self.need_to_redraw(camera_position) {
            self.build_voxels();
            self.vertices = get_vertices(&self.voxels, self.voxel_size);
        } 
        self.camera_position = camera_position;
        return self.vertices.clone();
    }
}
