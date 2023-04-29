use std::marker::PhantomData;

use crate::gl::core::instance::GL;
use crate::gl::core::utils::AsSlice;
use crate::gl::shader::locations::AttribLocation;
use crate::gl::shader::shader_use_ctx::ShaderUseCtx;

use super::object::GLObject;

const BINDED_OBJECT_ATTRIBUTES_INITIAL_CAPACITY: usize = 4;

pub struct BindedObjCtx<'a> {
    enabled_attributes: Vec<u32>,
    _phantom: PhantomData<&'a ()>,
}

impl BindedObjCtx<'_> {
    pub fn new(object: &GLObject) -> Self {
        GL.bind_vertex_array(Some(object.vao()));
        GL.bind_buffer(GL::ARRAY_BUFFER, Some(object.data()));
        GL.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(object.indices()));
        Self {
            enabled_attributes: Vec::with_capacity(BINDED_OBJECT_ATTRIBUTES_INITIAL_CAPACITY),
            _phantom: Default::default(),
        }
    }

    pub fn upload_static_array_buffer(&self, array: &impl AsSlice<f32>) {
        unsafe {
            GL.buffer_data_with_array_buffer_view(
                GL::ARRAY_BUFFER,
                &js_sys::Float32Array::view(array.as_slice()),
                GL::STATIC_DRAW,
            );
        }
    }

    pub fn upload_static_elem_buffer(&self, array: &impl AsSlice<i32>) {
        unsafe {
            GL.buffer_data_with_array_buffer_view(
                GL::ELEMENT_ARRAY_BUFFER,
                &js_sys::Int32Array::view(array.as_slice()),
                GL::STATIC_DRAW,
            );
        }
    }

    pub fn init_dyn_array_buffer(&self, size: usize) {
        GL.buffer_data_with_i32(GL::ARRAY_BUFFER, size as i32, GL::DYNAMIC_DRAW);
    }

    pub fn init_dyn_elem_buffer(&self, size: usize) {
        GL.buffer_data_with_i32(GL::ELEMENT_ARRAY_BUFFER, size as i32, GL::DYNAMIC_DRAW);
    }

    pub fn upload_sub_array(&self, array: &impl AsSlice<f32>, offset: usize) {
        unsafe {
            GL.buffer_sub_data_with_i32_and_array_buffer_view(
                GL::ARRAY_BUFFER,
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
            GL.enable_vertex_attrib_array(attribute_id);
            GL.vertex_attrib_pointer_with_i32(
                attribute_id,
                size as i32,
                GL::FLOAT,
                false,
                stride as i32,
                offset as i32,
            );
            self.enabled_attributes.push(attribute_id);
        }
    }

    pub fn draw_triangles(self, _: &ShaderUseCtx, count: usize) {
        GL.draw_elements_instanced_with_i32(
            GL::TRIANGLES,
            count as i32 * 3,
            GL::UNSIGNED_INT,
            0,
            3,
        );
    }
}

impl Drop for BindedObjCtx<'_> {
    fn drop(&mut self) {
        for &attribute_id in self.enabled_attributes.iter() {
            GL.disable_vertex_attrib_array(attribute_id);
        }
        GL.bind_buffer(GL::ARRAY_BUFFER, None);
        GL.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
        GL.bind_vertex_array(None);
    }
}
