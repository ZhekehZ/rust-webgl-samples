use glm::Mat4x4;

use crate::gl::shader::uniform_value::UniformValue;

use super::{
    core::instance::GL,
    shader::{
        locations::{SimpleUniformLocation, UniformLocation},
        program::ShaderProgram,
    },
};

pub struct Camera {
    pub model: Mat4x4,
    pub view: Mat4x4,
    pub projection: Mat4x4,
}

impl Camera {
    pub fn mvp(&self) -> Mat4x4 {
        self.projection * self.view * self.model
    }
}

pub struct CameraUniformLocation {
    model: SimpleUniformLocation<Mat4x4>,
    view: SimpleUniformLocation<Mat4x4>,
    projection: SimpleUniformLocation<Mat4x4>,
}

impl UniformLocation<Camera> for CameraUniformLocation {
    fn new(program: &ShaderProgram, name: &str) -> Self {
        let model_name = format!("{name}.model");
        let view_name = format!("{name}.view");
        let projection_name = format!("{name}.projection");
        Self {
            model: SimpleUniformLocation::new(program, model_name.as_str()),
            view: SimpleUniformLocation::new(program, view_name.as_str()),
            projection: SimpleUniformLocation::new(program, projection_name.as_str()),
        }
    }

    fn is_valid(&self) -> bool {
        self.model.is_valid() && self.view.is_valid() && self.projection.is_valid()
    }
}

impl UniformValue for Camera {
    type Location = CameraUniformLocation;

    fn set_to(&self, gl: &GL, location: &Self::Location) {
        self.model.set_to(gl, &location.model);
        self.view.set_to(gl, &location.view);
        self.projection.set_to(gl, &location.projection);
    }
}
