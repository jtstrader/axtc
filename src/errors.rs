use thiserror::Error;

#[derive(Error, Debug)]
pub enum AxtcError {
    #[error("provided color file does not exist")]
    FileNotFound,

    #[error("provided path is not a valid color file")]
    InvalidFileFormat,
}
