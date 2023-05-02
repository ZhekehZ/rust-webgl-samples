use thiserror::Error;

#[derive(Error, Debug)]
pub enum GLObjectError {
    #[error("Can't create VAO")]
    CreateVAOError,
    #[error("Can't create gl buffer")]
    CreateBufferError,
}
