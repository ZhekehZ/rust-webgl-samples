use js_sys::Object;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GLError {
    #[error(transparent)]
    GLObjectError(#[from] super::buffers::error::GLObjectError),   
    #[error(transparent)]
    ShaderError(#[from] super::shader::error::ShaderError),
    #[error("Cant't get webgl2 context. Value = {:?}", .0)]
    GL2ContextError(Object),
}