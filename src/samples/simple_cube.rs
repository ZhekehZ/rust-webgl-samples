use super::resources::simple_shaders::{FRAGMENT_SHADER, VERTEX_SHADER};
use crate::gl::camera::Camera;
use crate::gl::core::instance::GL;
use crate::gl::mesh::Mesh;
use crate::gl::shader::program::ShaderProgram;
use crate::gl::shader::shader_type::ShaderType;
use crate::samples::resources::cube_mesh::build_cube_mesh;
use glm::{look_at, perspective, Mat4x4, Vec3};
use wasm_bindgen::JsValue;

pub struct App {
    mesh: Mesh,
    shader: ShaderProgram,
    camera: Camera,
}

impl App {
    pub fn new() -> Result<App, JsValue> {
        GL.force_initialization();

        let mesh = build_cube_mesh();
        let shader = ShaderProgram::builder()
            .add_source(ShaderType::Vertex, VERTEX_SHADER.into())
            .add_source(ShaderType::Fragment, FRAGMENT_SHADER.into())
            .build()
            .unwrap();

        let model = glm::Mat4::new_scaling(0.25);
        let view = look_at(
            &Vec3::new(0.0, 0.0, 3.0),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        );
        let projection = perspective(0.74, std::f32::consts::FRAC_PI_3 * 2.0, 0.01, 10.0);
        let camera = Camera {
            model,
            view,
            projection,
        };

        Ok(Self {
            mesh,
            shader,
            camera,
        })
    }

    pub fn render(&mut self) -> Result<(), JsValue> {
        let width = GL.drawing_buffer_width();
        let height = GL.drawing_buffer_height();
        GL.viewport(0, 0, width, height);

        self.camera.model = glm::rotate(&self.camera.model, 0.01, &glm::Vec3::y());

        let aspect = width as f32 / height as f32;
        self.camera.projection =
            glm::perspective(aspect, std::f32::consts::FRAC_PI_3 * 2.0, 0.1, 10.0);

        GL.clear(GL::COLOR_BUFFER_BIT);
        GL.clear(GL::DEPTH_BUFFER_BIT);
        GL.clear_color(0.8, 0.9, 0.9, 1.0);

        let base_model = self.camera.model;
        for i in -4..=4 {
            for j in -4..=4 {
                let translation = glm::vec3(i as f32, j as f32, 0.0);
                self.camera.model = glm::Mat4::new_translation(&translation) * base_model;
                self.mesh.render(&self.shader, &self.camera);
            }
        }
        self.camera.model = base_model;

        Ok(())
    }
}
