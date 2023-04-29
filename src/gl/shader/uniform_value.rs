use crate::gl::core::instance::GL;
use glm::{Mat4x4, Vec2, Vec3};
use web_sys::WebGlUniformLocation;

use super::locations::{SimpleUniformLocation, UniformLocation};

pub trait UniformValue: Sized {
    type Location: UniformLocation<Self>;

    fn set_to(&self, location: &Self::Location);
}

pub trait SimpleUniformValue {
    fn simple_set_to(&self, location: Option<&WebGlUniformLocation>);
}

impl<T: SimpleUniformValue> UniformValue for T {
    type Location = SimpleUniformLocation<T>;

    fn set_to(&self, location: &SimpleUniformLocation<T>) {
        self.simple_set_to(location.loc.as_ref())
    }
}

impl SimpleUniformValue for f32 {
    fn simple_set_to(&self, location: Option<&WebGlUniformLocation>) {
        GL.uniform1f(location, *self)
    }
}

impl SimpleUniformValue for Vec2 {
    fn simple_set_to(&self, location: Option<&WebGlUniformLocation>) {
        GL.uniform2fv_with_f32_array(location, self.as_slice())
    }
}

impl SimpleUniformValue for Vec3 {
    fn simple_set_to(&self, location: Option<&WebGlUniformLocation>) {
        GL.uniform3fv_with_f32_array(location, self.as_slice())
    }
}

impl SimpleUniformValue for Mat4x4 {
    fn simple_set_to(&self, location: Option<&WebGlUniformLocation>) {
        GL.uniform_matrix4fv_with_f32_array(location, false, self.data.as_slice())
    }
}
