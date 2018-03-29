use game_application::cgmath::{Vector3, dot};

pub fn normalize(vector: &mut Vector3<f32>) {
    let length = dot(*vector, *vector);
    vector.x /= length;
    vector.y /= length;
    vector.z /= length;
}
