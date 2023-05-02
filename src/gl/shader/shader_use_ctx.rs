use crate::gl::core::instance::GL;

use super::program::ShaderProgram;
use super::uniform_value::UniformValue;

pub struct ShaderUseCtx<'a> {
    program: &'a ShaderProgram,
    gl: &'a GL,
}

impl<'a> ShaderUseCtx<'a> {
    pub fn new(gl: &'a GL, program: &'a ShaderProgram) -> Self {
        gl.use_program(Some(program.as_gl_program()));
        Self { program, gl }
    }

    pub fn as_program(&self) -> &ShaderProgram {
        self.program
    }

    pub fn set_uniform<V: UniformValue>(&self, location: &V::Location, value: &V) {
        value.set_to(self.gl, location)
    }
}

impl Drop for ShaderUseCtx<'_> {
    fn drop(&mut self) {
        self.gl.use_program(None);
    }
}
