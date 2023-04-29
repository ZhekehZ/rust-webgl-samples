use crate::gl::core::instance::GL;

use super::program::ShaderProgram;
use super::uniform_value::UniformValue;

pub struct ShaderUseCtx<'a> {
    program: &'a ShaderProgram,
}

impl<'a> ShaderUseCtx<'a> {
    pub fn new(program: &'a ShaderProgram) -> Self {
        GL.use_program(Some(program.as_gl_program()));
        ShaderUseCtx { program }
    }

    pub fn as_program(&self) -> &ShaderProgram {
        self.program
    }

    pub fn set_uniform<V: UniformValue>(&self, location: &V::Location, value: &V) {
        value.set_to(location)
    }
}

impl Drop for ShaderUseCtx<'_> {
    fn drop(&mut self) {
        GL.use_program(None);
    }
}
