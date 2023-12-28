use std::sync::Arc;

use rocket::futures::lock::Mutex;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

pub async fn gen_pool(url: &str) -> Arc<Mutex<SqlitePool>> {
    if !Sqlite::database_exists(url).await.unwrap_or(false) {
        println!("Creating database {}", url);
        match Sqlite::create_database(url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }
    let pool: SqlitePool = sqlx::SqlitePool::connect(url).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS products (id INTEGER PRIMARY KEY AUTOINCREMENT,name VARCHAR(50) NOT NULL,stock INTEGER NOT NULL DEFAULT 0);").execute(&pool).await.unwrap();
    Arc::from(Mutex::from(pool))
}
