use super::error::ShaderError;
use super::locations::{AttribLocation, UniformLocation};
use super::shader_type::ShaderType;
use super::shader_use_ctx::ShaderUseCtx;

use crate::gl;
use crate::gl::core::instance::GL;
use std::rc::Rc;

use super::uniform_value::UniformValue;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use web_sys::WebGlProgram;
use web_sys::WebGlShader;
use web_sys::WebGlUniformLocation;

#[derive(Debug, Clone)]
pub struct ShaderProgram {
    program: WebGlProgram,
    uid: u32,
    gl: Rc<GL>,
}

#[derive(Debug)]
pub struct ShaderProgramBuilder {
    shaders: Vec<(ShaderType, String)>,
    gl: Rc<GL>,
}

impl ShaderProgramBuilder {
    pub fn new(gl: &Rc<GL>) -> Self {
        Self {
            shaders: vec![],
            gl: Rc::clone(gl),
        }
    }

    pub fn add_source(mut self, shader_type: ShaderType, text: String) -> Self {
        self.shaders.push((shader_type, text));
        self
    }

    pub fn build(self) -> Result<ShaderProgram, ShaderError> {
        ShaderProgram::new(&self.gl, self.shaders.as_slice())
    }
}

impl ShaderProgram {
    pub fn builder(gl: &Rc<GL>) -> ShaderProgramBuilder {
        ShaderProgramBuilder::new(gl)
    }

    pub fn new(gl: &Rc<GL>, sources: &[(ShaderType, String)]) -> Result<Self, ShaderError> {
        static NEXT_UID: AtomicU32 = AtomicU32::new(0);

        let mut compiled_shaders = vec![];
        for (shader_type, text) in sources {
            compiled_shaders.push(compile_shader(gl, text, *shader_type)?);
        }

        let program = link_into_program(gl, &compiled_shaders)?;
        let uid = NEXT_UID.fetch_add(1, Ordering::SeqCst);

        Ok(ShaderProgram {
            program,
            uid,
            gl: Rc::clone(gl),
        })
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn as_gl_program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn get_raw_gl_uniform_location(&self, name: &str) -> Option<WebGlUniformLocation> {
        self.gl.get_uniform_location(&self.program, name)
    }

    pub fn get_uniform_location<V: UniformValue>(&self, name: &str) -> V::Location {
        let location = <V::Location>::new(self, name);
        if !location.is_valid() {
            log::warn!("Invalid uniform name: {}", name);
        }
        location
    }

    pub fn get_attrib_location(&self, name: &str) -> AttribLocation {
        AttribLocation::new(
            self,
            self.gl.get_attrib_location(&self.program, name) as u32,
        )
    }

    #[must_use]
    pub fn use_program(&self) -> ShaderUseCtx {
        ShaderUseCtx::new(&self.gl, self)
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program))
    }
}

struct CompiledShader<'a> {
    inner: WebGlShader,
    gl: &'a GL,
}

impl<'a> CompiledShader<'a> {
    fn new(gl: &'a GL, shader: WebGlShader) -> Self {
        log::trace!("New shader created");
        Self { inner: shader, gl }
    }
}

impl Drop for CompiledShader<'_> {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.inner));
        log::trace!("Shader deleted");
    }
}

fn compile_shader<'a>(
    gl: &'a GL,
    text: &str,
    shader_type: ShaderType,
) -> Result<CompiledShader<'a>, ShaderError> {
    let gl_shader = gl
        .create_shader(shader_type.to_gl_type())
        .ok_or(ShaderError::ShaderCreateError(shader_type))?;
    gl.shader_source(&gl_shader, text);
    gl.compile_shader(&gl_shader);
    let success = gl
        .get_shader_parameter(&gl_shader, gl::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);
    if success {
        Ok(CompiledShader::new(gl, gl_shader))
    } else {
        let logs = gl
            .get_shader_info_log(&gl_shader)
            .unwrap_or("Unknown error".into());
        Err(ShaderError::CompileError(logs, shader_type))
    }
}

fn link_into_program(gl: &GL, shaders: &[CompiledShader]) -> Result<WebGlProgram, ShaderError> {
    let gl_program = gl.create_program().ok_or(ShaderError::ProgramCreateError)?;
    for compiled_shader in shaders {
        gl.attach_shader(&gl_program, &compiled_shader.inner);
    }
    gl.link_program(&gl_program);
    let success = gl
        .get_program_parameter(&gl_program, gl::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);
    if success {
        Ok(gl_program)
    } else {
        let logs = gl
            .get_program_info_log(&gl_program)
            .unwrap_or("Unknown error".into());
        Err(ShaderError::LinkError(logs))
    }
}
