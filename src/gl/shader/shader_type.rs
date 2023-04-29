use crate::gl::core::instance::GL;

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    pub fn to_gl_type(self) -> u32 {
        match self {
            ShaderType::Vertex => GL::VERTEX_SHADER,
            ShaderType::Fragment => GL::FRAGMENT_SHADER,
        }
    }
}
