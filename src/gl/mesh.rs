use std::rc::Rc;

use na::{Matrix2xX, Matrix3xX};

use crate::gl;
use crate::gl::core::utils::SizeInBytes;
use crate::gl::shader::program::ShaderProgram;
use crate::math::compute_normals::compute_normals;

use super::buffers::object::GLObject;
use super::camera::Camera;
use super::core::instance::GL;
use super::error::GLError;

pub type Vertices = Matrix3xX<f32>;
pub type Normals = Matrix3xX<f32>;
pub type TexCoords = Matrix2xX<f32>;
pub type Faces = Matrix3xX<i32>;

pub struct MeshBuilder {
    vertices: Vertices,
    normals: Normals,
    tex_coords: TexCoords,
    faces: Faces,
}

impl MeshBuilder {
    pub fn new(vertices: Vertices, faces: Faces) -> Self {
        Self {
            vertices,
            normals: Normals::zeros(0),
            tex_coords: TexCoords::zeros(0),
            faces,
        }
    }

    pub fn add_normals(mut self, normals: Normals) -> Self {
        self.normals = normals;
        self
    }

    pub fn build_normals(self) -> Self {
        let normals = compute_normals(&self.vertices, &self.faces);
        self.add_normals(normals)
    }

    pub fn add_tex_coords(mut self, tex_coolds: TexCoords) -> Self {
        self.tex_coords = tex_coolds;
        self
    }

    pub fn build(self, gl: &Rc<GL>) -> Result<Mesh, GLError> {
        Mesh::try_new(gl, self.vertices, self.normals, self.tex_coords, self.faces)
    }
}

pub struct Mesh {
    vertices: Vertices,
    normals: Normals,
    tex_coords: TexCoords,
    faces: Faces,
    object: GLObject,
    gl: Rc<GL>,
}

impl Mesh {
    pub fn new_builder(vertices: Vertices, faces: Faces) -> MeshBuilder {
        MeshBuilder::new(vertices, faces)
    }

    pub fn try_new(
        gl: &Rc<GL>,
        vertices: Vertices,
        normals: Normals,
        tex_coords: TexCoords,
        faces: Faces,
    ) -> Result<Self, GLError> {
        let object = GLObject::try_new(gl)?;

        if let binded = object.bind() {
            let v_size = vertices.size_in_bytes();
            let n_size = vertices.size_in_bytes();
            let t_size = tex_coords.size_in_bytes();

            binded.init_dyn_array_buffer(v_size + n_size + t_size);
            binded.upload_sub_array(&vertices, 0);
            binded.upload_sub_array(&normals, v_size);
            binded.upload_sub_array(&tex_coords, v_size + n_size);
            binded.upload_static_elem_buffer(&faces);
        }

        Ok(Self {
            vertices,
            faces,
            normals,
            tex_coords,
            gl: Rc::clone(gl),
            object,
        })
    }

    pub fn update(&mut self, update: impl FnOnce(&mut Vertices, &mut Normals, &mut TexCoords)) {
        update(&mut self.vertices, &mut self.normals, &mut self.tex_coords);

        let v_size = self.vertices.size_in_bytes();
        let n_size = self.normals.size_in_bytes();

        let binded = self.object.bind();
        binded.upload_sub_array(&self.vertices, 0);
        binded.upload_sub_array(&self.normals, v_size);
        binded.upload_sub_array(&self.tex_coords, v_size + n_size);
    }

    pub fn render(&self, shader: &ShaderProgram, camera: &Camera) {
        self.gl.enable(gl::DEPTH_TEST);

        let shader_use = shader.use_program();

        let u_camera = shader.get_uniform_location::<Camera>("u_camera");
        shader_use.set_uniform(&u_camera, camera);

        let a_position = shader.get_attrib_location("a_position");
        let a_normal = shader.get_attrib_location("a_normal");
        let a_tex_coords = shader.get_attrib_location("a_tex_coords");

        if let mut binded = self.object.bind() {
            let v_size = self.vertices.size_in_bytes();
            let n_size = self.normals.size_in_bytes();
            let t_size = self.tex_coords.size_in_bytes();

            binded.vertex_attrib_pointer(&shader_use, a_position, 3, 0, 0);
            if n_size > 0 {
                binded.vertex_attrib_pointer(&shader_use, a_normal, 3, 0, v_size);
            }
            if t_size > 0 {
                binded.vertex_attrib_pointer(&shader_use, a_tex_coords, 2, 0, v_size + n_size);
            }

            binded.draw_triangles(&shader_use, self.faces.ncols());
        }

        self.gl.disable(gl::DEPTH_TEST);
    }
}
