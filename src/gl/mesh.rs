use na::Matrix3xX;

use crate::gl::core::utils::SizeInBytes;
use crate::gl::shader::program::ShaderProgram;
use crate::math::compute_normals::compute_normals;

use super::buffers::object::GLObject;
use super::camera::Camera;
use super::core::instance::GL;

pub struct Mesh {
    pub vertices: Matrix3xX<f32>,
    pub faces: Matrix3xX<i32>,
    object: GLObject,
}

impl Mesh {
    pub fn new(vertices: Matrix3xX<f32>, faces: Matrix3xX<i32>) -> Self {
        let normals = compute_normals(&vertices, &faces);
        let object = GLObject::try_new().unwrap();

        if let binded = object.bind() {
            binded.init_dyn_array_buffer(vertices.size_in_bytes() + normals.size_in_bytes());
            binded.upload_sub_array(&vertices, 0);
            binded.upload_sub_array(&normals, vertices.size_in_bytes());
            binded.upload_static_elem_buffer(&faces);
        }

        Self {
            vertices,
            faces,
            object,
        }
    }

    pub fn update_vertices(&mut self, update: impl FnOnce(&mut Matrix3xX<f32>)) {
        update(&mut self.vertices);
        let normals = compute_normals(&self.vertices, &self.faces);
        let binded = self.object.bind();
        binded.upload_sub_array(&self.vertices, 0);
        binded.upload_sub_array(&normals, self.vertices.size_in_bytes());
    }

    pub fn render(&self, shader: &ShaderProgram, camera: &Camera) {
        GL.enable(GL::DEPTH_TEST);

        let shader_use = shader.use_program();

        let u_camera = shader.get_uniform_location::<Camera>("u_camera");
        shader_use.set_uniform(&u_camera, camera);

        let a_position = shader.get_attrib_location("a_position");
        let a_normal = shader.get_attrib_location("a_normal");

        if let mut binded = self.object.bind() {
            binded.vertex_attrib_pointer(&shader_use, a_position, 3, 0, 0);
            binded.vertex_attrib_pointer(
                &shader_use,
                a_normal,
                3,
                0,
                self.vertices.size_in_bytes(),
            );
            binded.draw_triangles(&shader_use, self.faces.ncols());
        }

        GL.disable(GL::DEPTH_TEST);
    }
}