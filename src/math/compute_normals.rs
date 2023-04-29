use std::ops::AddAssign;

use na::{Matrix3xX, Vector3};

pub fn compute_normals(vertices: &Matrix3xX<f32>, faces: &Matrix3xX<i32>) -> Matrix3xX<f32> {
    let mut normals_data: Matrix3xX<f32> = Matrix3xX::zeros(vertices.ncols());

    for face in faces.column_iter() {
        let v0 = face[0] as usize;
        let v1 = face[1] as usize;
        let v2 = face[2] as usize;

        let a = vertices.column(v0);
        let b = vertices.column(v1);
        let c = vertices.column(v2);

        let ab = b - a;
        let ac = c - a;
        let normal: Vector3<f32> = ab.cross(&ac).normalize();

        normals_data.column_mut(v0).add_assign(&normal);
        normals_data.column_mut(v1).add_assign(&normal);
        normals_data.column_mut(v2).add_assign(&normal);
    }

    for mut column in normals_data.column_iter_mut() {
        column.copy_from(&column.normalize())
    }

    normals_data
}
