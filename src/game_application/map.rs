use game_application::vertex::Vertex;
use game_application::cgmath::{Point3, Vector3, dot, ElementWise};
use std::collections::{HashSet, HashMap};


pub struct Map {
    voxel_size: f32,
    map_size: (usize, usize, usize),
    voxels: Vec<Vec<Vec<bool>>>,
    position: (f32, f32, f32),
    vertices: Vec<Vertex>
}

type Edge = (Point3<usize>, Point3<usize>);


fn normalize(vector: &mut Vector3<f32>) {
    let length = dot(*vector, *vector);
    vector.div_element_wise(length);
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

    fn get_neighbors(&self, point: &Point3<usize>) -> HashSet<Point3<usize>> {
        let mut result = HashSet::new();
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
                            result.insert(neighbor);
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

    fn edge_neighbors(&self, edge: Edge) -> HashSet<Point3<usize>> {
        let first_neighbors = self.get_neighbors(&edge.0);
        let second_neighbors = self.get_neighbors(&edge.1);
        first_neighbors.intersection(&second_neighbors).cloned().collect()
    }

    fn extend_edge(&self, shape: &mut Vec<Vertex>, edge: Edge, visited_edges: &HashSet<Edge>, external_normals: &HashMap<Point3<usize>, Vector3<f32>>) -> Option<Edge> {
        let neighbors = self.edge_neighbors(edge);
        let mut have_next = false;
        let mut result = (Point3::new(0, 0, 0), Point3::new(0, 0, 0));
        for neighbor in &neighbors {
            if !external_normals.contains_key(neighbor) {
                continue;
            }
            if !have_next {
                if !visited_edges.contains(&(edge.0, *neighbor)) {
                    result.0 = edge.0;
                    result.1 = *neighbor;
                    have_next = true;
                }
                if !visited_edges.contains(&(edge.1, *neighbor)) {
                    result.0 = edge.1;
                    result.1 = *neighbor;
                    have_next = true;
                }
            }
            shape.push(Vertex{
                position: [(edge.0.x as f32) * self.voxel_size, (edge.0.z as f32) * self.voxel_size, (edge.0.y as f32) * self.voxel_size],
                normal: [external_normals[&edge.0].x, external_normals[&edge.0].z, external_normals[&edge.0].y],
                tex_coords: [0.0, 1.0]
            });
            shape.push(Vertex{
                position: [(edge.1.x as f32) * self.voxel_size, (edge.1.z as f32) * self.voxel_size, (edge.1.y as f32) * self.voxel_size],
                normal: [external_normals[&edge.1].x, external_normals[&edge.1].z, external_normals[&edge.1].y],
                tex_coords: [1.0, 1.0]
            });
            shape.push(Vertex{
                position: [(neighbor.x as f32) * self.voxel_size, (neighbor.z as f32) * self.voxel_size, (neighbor.y as f32) * self.voxel_size],
                normal: [external_normals[neighbor].x, external_normals[neighbor].z, external_normals[neighbor].y],
                tex_coords: [0.0, 0.0]
            });
        }
        if have_next {
            return Some(result);
        } else {
            return None;
        }
    }

    fn need_to_redraw(&self, position: (f32, f32, f32)) -> bool {
        self.vertices.len() == 0
    }

    pub fn get_vertices(&mut self, camera_position: (f32, f32, f32)) -> Vec<Vertex> {
        if self.need_to_redraw(camera_position) {
            let external_normals = self.get_external_normals();
            let mut visited_edges = HashSet::new();
            let mut edge = (Point3::new(0, 0, 0), Point3::new(0, 1, 0));
            visited_edges.insert(edge);

            let mut shape = Vec::new();
            loop {
                match self.extend_edge(&mut shape, edge, &visited_edges, &external_normals) {
                    Some(new_edge) => {
                        edge = new_edge;
                        visited_edges.insert(edge);
                    },
                    None => break
                }
            }
            self.vertices = shape.clone();
            return shape;
        } else {
            return self.vertices.clone();
        }
    }
}
