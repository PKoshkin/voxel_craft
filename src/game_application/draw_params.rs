use game_application::vertex::Vertex;


pub struct UniformsStruct {
}


pub struct DrawParams {
    pub shape: Vec<Vertex>,
    pub uniforms: UniformsStruct
}
