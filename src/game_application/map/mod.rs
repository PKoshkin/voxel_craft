use game_application::vertex::Vertex;
use game_application::cgmath::Point3;
use std::cmp;


pub struct Map {
    voxel_size: f32,
    map_size: (usize, usize, usize),
    voxels: Vec<Vec<Vec<bool>>> 
}


impl Map {
    pub fn new(voxel_size: f32, map_size: (usize, usize, usize)) -> Map {
        let mut voxels = Vec::new();
        for x in 0..map_size.0 {
            voxels.push(Vec::new());
            for y in 0..map_size.1 {
                voxels[x].push(Vec::new());
                for z in 0..map_size.2 {
                    voxels[x][y].push(false);
                }
            }
        }
        Map {
            voxel_size: voxel_size,
            map_size: map_size,
            voxels: voxels
        }
    }

    fn is_external(&self, x: usize, y: usize, z: usize) -> bool {
        if !(self.voxels[x][y][z]) {
            return false;
        }
        if ((x == 0) || (y == 0) || (z == 0) ||
            (x == self.map_size.0 - 1) || (y == self.map_size.1 - 1) || (z == self.map_size.2 - 1)) {
            return true;
        }
        !((self.voxels[x - 1][y][z]) &&
          (self.voxels[x + 1][y][z]) &&
          (self.voxels[x][y - 1][z]) &&
          (self.voxels[x][y + 1][z]) &&
          (self.voxels[x][y][z - 1]) &&
          (self.voxels[x][y][z + 1]))
    }

    fn get_external_voxels(&self) -> Vec<Vec<Vec<bool>>> {
        let mut external_voxels = self.voxels.clone();
        for x in 0..self.map_size.0 {
            for y in 0..self.map_size.1 {
                for z in 0..self.map_size.2 {
                    external_voxels[x][y][z] = self.is_external(x, y, z);
                }
            }
        }
        external_voxels
    }

    pub fn build_voxels(&mut self) {
        let high_map = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 2, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 2, 3, 2, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 2, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 3, 3, 3, 3, 3, 3, 3],
            vec![0, 0, 0, 3, 4, 4, 4, 4, 4, 3],
            vec![0, 0, 0, 3, 4, 5, 5, 5, 4, 3],
            vec![0, 0, 0, 3, 4, 5, 6, 5, 4, 3],
            vec![0, 0, 0, 3, 4, 5, 5, 5, 4, 3],
            vec![0, 0, 0, 3, 4, 4, 4, 4, 4, 3]
        ];
        for (y, vector) in high_map.iter().enumerate() {
            for (x, height) in vector.iter().enumerate() {
                for z in 0..(*height + 1) {
                    self.voxels[x][y][z] = true;
                }
            }
        }
    }

    pub fn add_vertices(&self, shape: &mut Vec<Vertex>, indices: &mut Vec<u16>) {
        let external_voxels = self.get_external_voxels();
        for x in 0..self.map_size.0 {
            for y in 0..self.map_size.1 {
                for z in 0..self.map_size.2 {
                    if external_voxels[x][y][z] {
                        shape.push(Vertex{
                            position: [(x as f32) * self.voxel_size, (z as f32) * self.voxel_size, (y as f32) * self.voxel_size],
                            normal: [0.0, 0.0, -1.0],
                            tex_coords: [0.0, 1.0]
                        });
                    }
                }
            }
        }
        indices.extend_from_slice(&[
            0, 1, 2,
            1, 2, 3
        ]);
    }
}
