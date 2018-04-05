use game_application::cgmath::{Vector3, Point3};


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(point: &Point3<usize>, normal: &Vector3<f32>, tex_coords: &[f32; 2], voxel_size: f32) -> Vertex {
        Vertex{
            position: [(point.x as f32) * voxel_size / 2.0, (point.z as f32) * voxel_size / 2.0, (point.y as f32) * voxel_size / 2.0],
            normal: [normal.x, normal.y, normal.z],
            tex_coords: *tex_coords
        }
    }
}

implement_vertex!(Vertex, position, normal, tex_coords);
