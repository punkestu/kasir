use std::{fmt::Display, result};

#[derive(Responder)]
#[response(status = 500)]
pub struct InternalServerError(String);
impl InternalServerError {
    pub fn from<E: Display>(e: E) -> Self {
        Self(e.to_string())
    }
}
impl Default for InternalServerError {
    fn default() -> Self {
        InternalServerError("something wrong in backend".into())
    }
}

pub mod index;
pub mod page;
pub type Result<T> = result::Result<T, InternalServerError>;
