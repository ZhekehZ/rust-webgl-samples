use std::rc::Rc;

use na::Matrix3xX;

use crate::{
    faces,
    gl::{core::instance::GL, error::GLError, mesh::Mesh},
    vertices,
};

pub fn get_cube_vertices() -> Matrix3xX<f32> {
    vertices![
        -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0,
        1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0,
        -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
    ]
}

pub fn get_cube_faces() -> Matrix3xX<i32> {
    faces![
        0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4, 8, 9, 10, 10, 11, 8, 12, 13, 14, 14, 15, 12, 16, 17,
        18, 18, 19, 16, 20, 21, 22, 22, 23, 20,
    ]
}

pub fn build_cube_mesh(gl: &Rc<GL>) -> Result<Mesh, GLError> {
    Mesh::new_builder(get_cube_vertices(), get_cube_faces())
        .build_normals()
        .build(gl)
}
