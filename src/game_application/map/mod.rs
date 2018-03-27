use game_application::vertex::Vertex;
use game_application::cgmath::{Point3, Vector3, dot, ElementWise};
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;


pub struct Map {
    voxel_size: f32,
    map_size: (usize, usize, usize),
    voxels: Vec<Vec<Vec<bool>>> 
}


fn length(vector: Vector3<f32>) -> f32 {
    dot(vector, vector).sqrt()
}


fn normalize(vector: &mut Vector3<f32>) {
    let length = dot(*vector, *vector);
    vector.div_element_wise(length);
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

    fn is_external(&self, point: &Point3<usize>) -> bool {
        if !(self.voxels[point.x][point.y][point.z]) {
            return false;
        }
        if (point.x == 0) ||
           (point.y == 0) ||
           (point.z == 0) ||
           (point.x == self.map_size.0 - 1) ||
           (point.y == self.map_size.1 - 1) ||
           (point.z == self.map_size.2 - 1) {
            return true;
        }
        !((self.voxels[point.x - 1][point.y][point.z]) &&
          (self.voxels[point.x + 1][point.y][point.z]) &&
          (self.voxels[point.x][point.y - 1][point.z]) &&
          (self.voxels[point.x][point.y + 1][point.z]) &&
          (self.voxels[point.x][point.y][point.z - 1]) &&
          (self.voxels[point.x][point.y][point.z + 1]))
    }

    fn is_in_map(&self, point: &Point3<i32>) -> bool {
        ((point.x >= 0) &&
         (point.x < self.map_size.0 as i32) &&
         (point.y >= 0) &&
         (point.y < self.map_size.1 as i32) &&
         (point.z >= 0) &&
         (point.z < self.map_size.2 as i32))
    }

    fn get_neighbors(&self, point: &Point3<usize>) -> Vec<Point3<usize>> {
        let mut result = Vec::new();
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if (dx == 0) && (dy == 0) && (dz == 0) {
                        continue;
                    }
                    let neighbor = Point3::new(point.x as i32 + dx, point.y as i32 + dy, point.z as i32 + dz);
                    if self.is_in_map(&neighbor) {
                        let neighbor = neighbor.cast::<usize>().unwrap();
                        if self.voxels[neighbor.x][neighbor.y][neighbor.z] {
                            result.push(neighbor);
                        }
                    }
                }
            }
        }
        result
    }

    fn get_external_normals(&self) -> HashMap<Point3<usize>, Vector3<f32>> {
        let mut result = HashMap::new();
        for x in 0..self.map_size.0 {
            for y in 0..self.map_size.1 {
                for z in 0..self.map_size.2 {
                    let point = Point3::new(x, y, z);
                    if self.is_external(&point) {
                        let neighbors = self.get_neighbors(&point);
                        let mut normal = Vector3::new(0.0, 0.0, 0.0);
                        for neighbor in neighbors.iter() {
                            normal += point.cast::<f32>().unwrap() - neighbor.cast::<f32>().unwrap();
                        }
                        normalize(&mut normal);
                        result.insert(point, normal);
                    }
                }
            }
        }
        result
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

    pub fn add_vertices(&self, shape: &mut Vec<Vertex>) {
        let external_normals = self.get_external_normals();
        let mut visited = HashSet::new();
        for (voxel, voxel_normal) in external_normals.iter() {
            if visited.contains(voxel) {
                continue;
            }

            let mut neighbors = self.get_neighbors(voxel);
            for neighbor in &neighbors {
                visited.insert(neighbor.clone());
            }
            let normal_to_plane = voxel_normal.cross(neighbors[0].cast::<f32>().unwrap() - voxel.cast::<f32>().unwrap());

            neighbors.sort_by(|point1, point2| {
                let a = normal_to_plane;
                let b1 = point1.cast::<f32>().unwrap() - voxel.cast::<f32>().unwrap();
                let sin1 = dot(a, b1) / (length(a) * length(b1));
                let b2 = point2.cast::<f32>().unwrap() - voxel.cast::<f32>().unwrap();
                let sin2 = dot(a, b2) / (length(a) * length(b2));
                if sin1.asin() < sin2.asin() {
                    return Ordering::Less;
                }  else {
                    return Ordering::Greater;
                }
            });
            for i in 1..neighbors.len() {
                shape.push(Vertex{
                    position: [(voxel.x as f32) * self.voxel_size, (voxel.z as f32) * self.voxel_size, (voxel.y as f32) * self.voxel_size],
                    normal: [voxel_normal.x, voxel_normal.y, voxel_normal.z],
                    tex_coords: [0.0, 1.0]
                });
                shape.push(Vertex{
                    position: [(neighbors[i - 1].x as f32) * self.voxel_size, (neighbors[i - 1].z as f32) * self.voxel_size, (neighbors[i - 1].y as f32) * self.voxel_size],
                    normal: [voxel_normal.x, voxel_normal.y, voxel_normal.z],
                    tex_coords: [1.0, 1.0]
                });
                shape.push(Vertex{
                    position: [(neighbors[i].x as f32) * self.voxel_size, (neighbors[i].z as f32) * self.voxel_size, (neighbors[i].y as f32) * self.voxel_size],
                    normal: [voxel_normal.x, voxel_normal.y, voxel_normal.z],
                    tex_coords: [0.0, 0.0]
                });
            }
            visited.insert(voxel.clone());
        }
    }
}
