use crate::gl::{core::instance::GL, error::GLError};

pub trait Sample: Sized {
    fn try_new(gl: GL) -> Result<Self, GLError>;
    
    fn render(&mut self) -> Result<(), GLError>;
}
