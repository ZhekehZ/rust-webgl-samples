use crate::gl::core::instance::GL;
use web_sys::{WebGlBuffer, WebGlVertexArrayObject};

use super::binded_obj_ctx::BindedObjCtx;
use super::object_error::GLObjectError::{self, CreateBufferError, CreateVAOError};

pub struct GLObject {
    vertex_array_object: WebGlVertexArrayObject,
    data_buffer: WebGlBuffer,
    index_buffer: WebGlBuffer,
}

impl GLObject {
    pub fn try_new() -> Result<Self, GLObjectError> {
        let vertex_array_object = GL.create_vertex_array().ok_or(CreateVAOError)?;
        let data_buffer = GL.create_buffer().ok_or(CreateBufferError)?;
        let index_buffer = GL.create_buffer().ok_or(CreateBufferError)?;
        Ok(Self {
            vertex_array_object,
            data_buffer,
            index_buffer,
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
        BindedObjCtx::new(self)
    }
}

impl Drop for GLObject {
    fn drop(&mut self) {
        GL.delete_buffer(Some(&self.data_buffer));
        GL.delete_buffer(Some(&self.index_buffer));
        GL.delete_vertex_array(Some(&self.vertex_array_object));
    }
}
