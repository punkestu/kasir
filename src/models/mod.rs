use std::{error, fmt::Display, result, sync::Arc};

use rocket::futures::lock::Mutex;
use tera::Tera;

use self::product::ProductRepo;

pub mod product;
pub mod repo;

#[derive(Debug)]
pub struct Error {
    pub cause: String,
    pub currently_on: String,
}

impl Error {
    pub fn from<E: error::Error>(err: E, currently_on: String) -> Self {
        Self {
            cause: err.to_string(),
            currently_on,
        }
    }

    pub fn map<E: error::Error>(currently_on: String) -> impl FnOnce(E) -> Self {
        |err: E| -> Self { Self::from(err, currently_on) }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on {}", self.cause, self.currently_on)
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub struct AppState {
    pub product_repo: Arc<Mutex<dyn ProductRepo + Send>>,
    pub tera: Arc<Tera>,
}
