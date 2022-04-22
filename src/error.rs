use actix_web::{ResponseError};
use thiserror::Error;

#[derive(Error,Debug)]
pub enum MyError {
    #[error("Failed")]
    AskmaError(#[from] askama::Error),

    #[error("Failed connection")]
    ConncectionPoolError(#[from] r2d2::Error),

}

impl ResponseError for MyError {}