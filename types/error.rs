use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnumError {
    #[error("Failed to parse either_bool {0}")]
    EitherBool(String),
    #[error("Failed to parse config weight {0}")]
    ConfigWeight(String),
    #[error("Failed to parse difficulty {0}")]
    Difficulty(String),
}

#[derive(Error, Debug)]
pub enum VecError {
    #[error("Empty monster vector for level {0}")]
    EmptyMonster(i32),
}
