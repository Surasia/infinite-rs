use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("Incorrect magic found! Expected 'mohd', found {0}!")]
    IncorrectMagic(String),
    #[error("Incorrect version found! Expected '53', found {0}!")]
    IncorrectVersion(i32),
}

#[derive(Error, Debug)]
pub enum TagError {
    #[error("Incorrect magic found! Expected 'ucsh', found {0}!")]
    IncorrectMagic(String),
    #[error("Incorrect version found! Expected '27', found {0}!")]
    IncorrectVersion(i32),
}
