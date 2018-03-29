use game_application::vertex::Vertex;
use game_application::marching_cubes::get_vertices;


pub struct Map {
    voxel_size: f32,
    map_size: (usize, usize, usize),
    voxels: Vec<Vec<Vec<bool>>>,
    position: (f32, f32, f32),
    vertices: Vec<Vertex>
}


impl Map {
    pub fn new(voxel_size: f32, map_size: (usize, usize, usize), camera_position: (f32, f32, f32)) -> Map {
        let mut voxels = Vec::new();
        for x in 0..map_size.0 {
            voxels.push(Vec::new());
            for y in 0..map_size.1 {
                voxels[x].push(Vec::new());
                for _ in 0..map_size.2 {
                    voxels[x][y].push(false);
                }
            }
        }
        Map {
            voxel_size: voxel_size,
            map_size: map_size,
            voxels: voxels,
            position: camera_position,
            vertices: Vec::new()
        }
    }

    pub fn build_voxels(&mut self) {
        let high_map = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 2, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 2, 3, 2, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 2, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 3, 3, 3, 3, 3, 3, 0],
            vec![0, 0, 0, 3, 4, 4, 4, 4, 4, 0],
            vec![0, 0, 0, 3, 4, 5, 5, 5, 4, 0],
            vec![0, 0, 0, 3, 4, 5, 6, 5, 4, 0],
            vec![0, 0, 0, 3, 4, 5, 5, 5, 4, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        ];
        for (y, vector) in high_map.iter().enumerate() {
            for (x, height) in vector.iter().enumerate() {
                for z in 0..(*height + 1) {
                    self.voxels[x][y][z] = true;
                }
            }
        }
    }


    fn need_to_redraw(&self, position: (f32, f32, f32)) -> bool {
        self.vertices.len() == 0
    }

    pub fn get_vertices(&mut self, camera_position: (f32, f32, f32)) -> Vec<Vertex> {
        if self.need_to_redraw(camera_position) {
            self.build_voxels();
            self.vertices = get_vertices(&self.voxels, self.voxel_size);
        } 
        self.position = camera_position;
        return self.vertices.clone();
    }
}
