use game_application::vertex::Vertex;


pub struct UniformsStruct {
}


pub struct DrawParams {
    pub shape: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub uniforms: UniformsStruct
}
