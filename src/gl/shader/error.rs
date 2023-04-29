use super::shader_type::ShaderType;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum ShaderError {
    ShaderCreateError(ShaderType),
    CompileError(String, ShaderType),
    ProgramCreateError,
    LinkError(String),
}
