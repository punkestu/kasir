use std::sync::Arc;

use rocket::futures::lock::Mutex;

use self::product::ProductRepo;

pub mod product;
pub mod repo;

pub struct AppState {
    pub product_repo: Arc<Mutex<dyn ProductRepo + Send>>,
}
