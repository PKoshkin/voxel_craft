use game_application::cgmath::{Vector3, dot, ElementWise};

pub fn normalize(vector: &mut Vector3<f32>) {
    let length = dot(*vector, *vector).sqrt();
    vector.div_assign_element_wise(length);
}
