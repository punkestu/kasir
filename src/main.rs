#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use models::repo::sqlite;
use rocket::{fs::FileServer, futures::lock::Mutex};
use std::sync::Arc;
use tera::Tera;

use crate::models::AppState;

mod db;
mod functions;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let db_url = std::env::var("DB_URL").expect("$DB_URL is not set");
    let db = db::sqlite::gen_pool(db_url.as_str()).await;
    let product_repo = sqlite::ProductRepo::new(db);

    rocket::build()
        .manage(AppState {
            product_repo: Arc::from(Mutex::from(product_repo)),
            tera: Arc::from(Tera::new("client/views/**/*.tera").expect("failed to run tera")),
        })
        .mount("/", routes![routes::index::index])
        .mount("/public", FileServer::from("client/public"))
        .launch()
        .await?;
    Ok(())
}
