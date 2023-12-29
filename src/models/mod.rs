use std::{result, sync::Arc};

use rocket::futures::lock::Mutex;
use tera::Tera;

use self::product::ProductRepo;

pub mod product;
pub mod repo;

pub struct Error {
    pub code: u16,
    pub cause: String,
    pub currently_on: String,
}
pub type Result<T> = result::Result<T, Error>;

pub struct AppState {
    pub product_repo: Arc<Mutex<dyn ProductRepo + Send>>,
    pub tera: Arc<Mutex<Tera>>,
}
