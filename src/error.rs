use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type TreeResult<T> = Result<T, TreeError>;
