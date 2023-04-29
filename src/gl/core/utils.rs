use na::{self, Dim, IsContiguous, Matrix, RawStorage};

pub trait SizeInBytes {
    fn size_in_bytes(&self) -> usize;
}

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

impl<T, A: Dim, B: Dim, C: RawStorage<T, A, B> + IsContiguous> AsSlice<T> for Matrix<T, A, B, C> {
    fn as_slice(&self) -> &[T] {
        (self as &Matrix<T, A, B, C>).as_slice()
    }
}

impl<T, A: Dim, B: Dim, C: RawStorage<T, A, B> + IsContiguous> SizeInBytes for Matrix<T, A, B, C> {
    fn size_in_bytes(&self) -> usize {
        self.as_slice().len() * std::mem::size_of::<T>()
    }
}
