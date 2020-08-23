use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Empty description file")]
    EmptyFile,

    #[error("Incorrect shape description")]
    IncorrectShape,

    #[error("Incorrect grid description")]
    IncorrectGrid,
}
