use std::sync::Arc;

use crate::models::product::Product;

use super::super::super::product;
use rocket::futures::lock::Mutex;
use sqlx::{Row, SqlitePool};
pub struct ProductRepo {
    pool: Arc<Mutex<SqlitePool>>,
}

impl ProductRepo {
    pub fn new(pool: Arc<Mutex<SqlitePool>>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl product::ProductRepo for ProductRepo {
    async fn get_all(&self) -> Vec<product::Product> {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        let products = sqlx::query("SELECT * FROM products")
            .fetch_all(&(*pool))
            .await
            .unwrap();
        drop(pool);

        products
            .iter()
            .map(|product| Product {
                id: Some(product.get::<i64, _>("id") as u64),
                name: product.get::<String, _>("name"),
                stock: product.get::<i32, _>("stock") as u32,
            })
            .collect()
    }
    async fn save(&self, mut product: Product) -> Product {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        match product.id {
            Some(id) => {
                let update_product = product.clone();
                sqlx::query("UPDATE products SET name=?, stock=? WHERE id=?")
                    .bind(update_product.name)
                    .bind(update_product.stock)
                    .bind(id as i64)
                    .execute(&(*pool))
                    .await
                    .unwrap();
            }
            None => {
                let new_product = product.clone();
                product.id = Some(
                    sqlx::query("INSERT INTO products(name, stock) VALUES(?,?)")
                        .bind(new_product.name)
                        .bind(new_product.stock)
                        .execute(&(*pool))
                        .await
                        .unwrap()
                        .last_insert_rowid() as u64,
                );
            }
        };
        drop(pool);
        product
    }
    async fn delete(&self, id: u64) -> bool {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        sqlx::query("DELETE FROM products WHERE id=?")
            .bind(id as i64)
            .execute(&(*pool))
            .await
            .unwrap();
        drop(pool);
        true
    }
}
