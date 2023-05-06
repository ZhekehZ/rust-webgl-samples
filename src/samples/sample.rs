use crate::gl::{core::instance::GL, error::GLError};

pub trait Sample: Sized {
    fn try_new(gl: GL) -> Result<Self, GLError>;

    fn update(&mut self, d_time: f64) -> Result<(), GLError>;

    fn render(&mut self) -> Result<(), GLError>;
}
