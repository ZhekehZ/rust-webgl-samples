use std::marker::PhantomData;

use web_sys::WebGlUniformLocation;

use crate::gl::core::instance::GL;

use super::{program::ShaderProgram, uniform_value::UniformValue};

pub trait UniformLocation<V: UniformValue<Location = Self>> {
    fn new(program: &ShaderProgram, name: &str) -> Self;

    fn is_valid(&self) -> bool {
        true
    }
}

pub struct SimpleUniformLocation<V> {
    pub loc: Option<WebGlUniformLocation>,
    _phantom: PhantomData<V>,
}

pub struct AttribLocation {
    program_uid: u32,
    location: u32,
}

impl<V: UniformValue<Location = Self>> UniformLocation<V> for SimpleUniformLocation<V> {
    fn new(program: &ShaderProgram, name: &str) -> Self {
        let loc = GL.get_uniform_location(program.as_gl_program(), name);
        Self {
            loc,
            _phantom: Default::default(),
        }
    }

    fn is_valid(&self) -> bool {
        self.loc.is_some()
    }
}

impl AttribLocation {
    pub fn new(program: &ShaderProgram, location: u32) -> Self {
        Self {
            program_uid: program.uid(),
            location,
        }
    }

    pub fn try_inner_for(&self, program: &ShaderProgram) -> Option<u32> {
        (self.program_uid == program.uid()).then_some(self.location)
    }
}
