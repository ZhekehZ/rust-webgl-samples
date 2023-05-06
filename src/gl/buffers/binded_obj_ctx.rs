use crate::gl;
use crate::gl::core::instance::GL;
use crate::gl::core::utils::AsSlice;
use crate::gl::shader::locations::AttribLocation;
use crate::gl::shader::shader_use_ctx::ShaderUseCtx;

use super::object::GLObject;

const BINDED_OBJECT_ATTRIBUTES_INITIAL_CAPACITY: usize = 4;

pub struct BindedObjCtx<'a> {
    enabled_attributes: Vec<u32>,
    gl: &'a GL,
}

impl<'a> BindedObjCtx<'a> {
    pub fn new(gl: &'a GL, object: &'a GLObject) -> Self {
        gl.bind_vertex_array(Some(object.vao()));
        gl.bind_buffer(gl::ARRAY_BUFFER, Some(object.data()));
        gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(object.indices()));
        Self {
            enabled_attributes: Vec::with_capacity(BINDED_OBJECT_ATTRIBUTES_INITIAL_CAPACITY),
            gl,
        }
    }

    pub fn upload_static_array_buffer(&self, array: &impl AsSlice<f32>) {
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                gl::ARRAY_BUFFER,
                &js_sys::Float32Array::view(array.as_slice()),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn upload_static_elem_buffer(&self, array: &impl AsSlice<i32>) {
        unsafe {
self.gl.buffer_data_with_array_buffer_view(
                gl::ELEMENT_ARRAY_BUFFER,
                &js_sys::Int32Array::view(array.as_slice()),
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn init_dyn_array_buffer(&self, size: usize) {
        self.gl
            .buffer_data_with_i32(gl::ARRAY_BUFFER, size as i32, gl::DYNAMIC_DRAW);
    }

    pub fn upload_sub_array(&self, array: &impl AsSlice<f32>, offset: usize) {
        unsafe {
            self.gl.buffer_sub_data_with_i32_and_array_buffer_view(
                gl::ARRAY_BUFFER,
                offset as i32,
                &js_sys::Float32Array::view(array.as_slice()),
            );
        }
    }

    pub fn vertex_attrib_pointer(
        &mut self,
        shader_use: &ShaderUseCtx,
        attribute: AttribLocation,
        size: usize,
        stride: usize,
        offset: usize,
    ) {
        if let Some(attribute_id) = attribute.try_inner_for(shader_use.as_program()) {
            self.gl.enable_vertex_attrib_array(attribute_id);
            self.gl.vertex_attrib_pointer_with_i32(
                attribute_id,
                size as i32,
                gl::FLOAT,
                false,
                stride as i32,
                offset as i32,
            );
            self.enabled_attributes.push(attribute_id);
        }
    }

    pub fn draw_triangles(self, _: &ShaderUseCtx, count: usize) {
        self.gl.draw_elements_instanced_with_i32(
            gl::TRIANGLES,
            count as i32 * 3,
            gl::UNSIGNED_INT,
            0,
            3,
        );
    }
}

impl Drop for BindedObjCtx<'_> {
    fn drop(&mut self) {
        for &attribute_id in self.enabled_attributes.iter() {
            self.gl.disable_vertex_attrib_array(attribute_id);
        }
        self.gl.bind_buffer(gl::ARRAY_BUFFER, None);
        self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, None);
        self.gl.bind_vertex_array(None);
    }
}
