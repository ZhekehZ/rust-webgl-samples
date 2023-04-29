#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}

#[macro_export]
macro_rules! matrix_xx3 {
    ($t:ty; $($x:expr, $y:expr, $z: expr),+ $(,)?) => (
        {
            let vertices_count = $crate::count!($($x)*);
            let mut data = na::Matrix3xX::<$t>::zeros(vertices_count);
            let mut _col_idx = 0;
            $(
                data[(0, _col_idx)] = $x;
                data[(1, _col_idx)] = $y;
                data[(2, _col_idx)] = $z;
                _col_idx += 1;
                )*
            data
        }
    );
}

#[macro_export]
macro_rules! vertices {
    () => ($crate::matrix_xx3![f32;]);
    ($fst:expr $(, $rest:expr)* $(,)?) => ($crate::matrix_xx3![f32; $fst $(, $rest)*])
}

#[macro_export]
macro_rules! faces {
    () => ($crate::matrix_xx3![i32;]);
    ($fst:expr $(, $rest:expr)* $(,)?) => ($crate::matrix_xx3![i32; $fst $(, $rest)*])
}
