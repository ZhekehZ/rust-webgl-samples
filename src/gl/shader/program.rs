use super::error::ShaderError;
use super::locations::{AttribLocation, UniformLocation};
use super::shader_type::ShaderType;
use super::shader_use_ctx::ShaderUseCtx;

use crate::gl::core::instance::GL;

use super::uniform_value::UniformValue;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    program: WebGlProgram,
    uid: u32,
}

#[derive(Debug)]
pub struct ShaderProgramBuilder {
    shaders: Vec<(ShaderType, String)>,
}

impl ShaderProgramBuilder {
    pub fn new() -> Self {
        Self { shaders: vec![] }
    }

    pub fn add_source(mut self, shader_type: ShaderType, text: String) -> Self {
        self.shaders.push((shader_type, text));
        self
    }

    pub fn build(self) -> Result<ShaderProgram, ShaderError> {
        ShaderProgram::new(self.shaders.as_slice())
    }
}

impl ShaderProgram {
    pub fn builder() -> ShaderProgramBuilder {
        ShaderProgramBuilder::new()
    }

    pub fn new(sources: &[(ShaderType, String)]) -> Result<Self, ShaderError> {
        static NEXT_UID: AtomicU32 = AtomicU32::new(0);

        let mut compiled_shaders = vec![];
        for (shader_type, text) in sources {
            compiled_shaders.push(compile_shader(text, *shader_type)?);
        }

        let program = link_into_program(&compiled_shaders)?;
        let uid = NEXT_UID.fetch_add(1, Ordering::SeqCst);

        Ok(ShaderProgram { program, uid })
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn as_gl_program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn get_uniform_location<V: UniformValue>(&self, name: &str) -> V::Location {
        let location = <V::Location>::new(self, name);
        if !location.is_valid() {
            log::warn!("Invalid uniform name: {}", name);
        }
        location
    }

    pub fn get_attrib_location(&self, name: &str) -> AttribLocation {
        AttribLocation::new(self, GL.get_attrib_location(&self.program, name) as u32)
    }

    #[must_use]
    pub fn use_program(&self) -> ShaderUseCtx {
        ShaderUseCtx::new(self)
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        GL.delete_program(Some(&self.program))
    }
}

struct CompiledShader(WebGlShader);

impl CompiledShader {
    fn new(shader: WebGlShader) -> Self {
        log::trace!("New shader created");
        Self(shader)
    }
}

impl Drop for CompiledShader {
    fn drop(&mut self) {
        GL.delete_shader(Some(&self.0));
        log::trace!("Shader deleted");
    }
}

fn compile_shader(text: &str, shader_type: ShaderType) -> Result<CompiledShader, ShaderError> {
    let gl_shader = GL
        .create_shader(shader_type.to_gl_type())
        .ok_or(ShaderError::ShaderCreateError(shader_type))?;
    GL.shader_source(&gl_shader, text);
    GL.compile_shader(&gl_shader);
    let success = GL
        .get_shader_parameter(&gl_shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);
    if success {
        Ok(CompiledShader::new(gl_shader))
    } else {
        let logs = GL
            .get_shader_info_log(&gl_shader)
            .unwrap_or("Unknown error".into());
        Err(ShaderError::CompileError(logs, shader_type))
    }
}

fn link_into_program(shaders: &[CompiledShader]) -> Result<WebGlProgram, ShaderError> {
    let gl_program = GL.create_program().ok_or(ShaderError::ProgramCreateError)?;
    for CompiledShader(shader) in shaders {
        GL.attach_shader(&gl_program, shader);
    }
    GL.link_program(&gl_program);
    let success = GL
        .get_program_parameter(&gl_program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);
    if success {
        Ok(gl_program)
    } else {
        let logs = GL
            .get_program_info_log(&gl_program)
            .unwrap_or("Unknown error".into());
        Err(ShaderError::LinkError(logs))
    }
}
