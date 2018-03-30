use game_application::cgmath::{Vector3, dot, ElementWise};

pub fn normalize(vector: &mut Vector3<f32>) {
    let length = dot(*vector, *vector).sqrt();
    vector.div_assign_element_wise(length);
}

pub fn get_normalized(vector: Vector3<f32>) -> Vector3<f32> {
    let length = dot(vector, vector).sqrt();
    Vector3::new(vector.x / length, vector.y / length, vector.z / length)
}
