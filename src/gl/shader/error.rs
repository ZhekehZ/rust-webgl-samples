use super::shader_type::ShaderType;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Error, Debug, Clone)]
pub enum ShaderError {
    #[error("Can't create {} shader", simple_shader_type_name(.0))]
    ShaderCreateError(ShaderType),
    #[error("Can't compile {} shader. Message: {0}", simple_shader_type_name(.1))]
    CompileError(String, ShaderType),
    #[error("Can't create shader program")]
    ProgramCreateError,
    #[error("Can't link shaders into program. Message: {0}")]
    LinkError(String),
}

fn simple_shader_type_name(t: &ShaderType) -> String {
    match *t {
        ShaderType::Vertex => "vertex",
        ShaderType::Fragment => "fragment",
    }
    .into()
}
