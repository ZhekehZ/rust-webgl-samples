use std::rc::Rc;

use crate::gl::core::instance::GL;
use web_sys::{WebGlBuffer, WebGlVertexArrayObject};

use super::binded_obj_ctx::BindedObjCtx;
use super::error::GLObjectError::{self, CreateBufferError, CreateVAOError};

pub struct GLObject {
    vertex_array_object: WebGlVertexArrayObject,
    data_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
    gl: Rc<GL>,
}

impl GLObject {
    pub fn try_new(gl: &Rc<GL>) -> Result<Self, GLObjectError> {
        let vertex_array_object = gl.create_vertex_array().ok_or(CreateVAOError)?;
        let data_buffer = gl.create_buffer().ok_or(CreateBufferError)?;
        let index_buffer = gl.create_buffer().ok_or(CreateBufferError)?;
        Ok(Self {
            vertex_array_object,
            data_buffer,
            index_buffer,
            gl: Rc::clone(gl),
        })
    }

    pub fn vao(&self) -> &WebGlVertexArrayObject {
        &self.vertex_array_object
    }

    pub fn data(&self) -> &WebGlBuffer {
        &self.data_buffer
    }

    pub fn indices(&self) -> &WebGlBuffer {
        &self.index_buffer
    }

    #[must_use]
    pub fn bind(&self) -> BindedObjCtx {
        BindedObjCtx::new(self.gl.as_ref(), self)
    }
}

impl Drop for GLObject {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(&self.data_buffer));
        self.gl.delete_buffer(Some(&self.index_buffer));
        self.gl.delete_vertex_array(Some(&self.vertex_array_object));
    }
}
