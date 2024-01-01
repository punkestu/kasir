use std::sync::Arc;

use crate::models::{product::Product, Error};

use super::super::super::{product, Result};
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
    async fn get_all(&self) -> Result<Vec<product::Product>> {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        let products = sqlx::query("SELECT * FROM products")
            .fetch_all(&(*pool))
            .await
            .map_err(|err| -> Error {
                Error {
                    cause: err.to_string(),
                    currently_on: "product get_all()".into(),
                }
            })?;
        drop(pool);

        Ok(products
            .iter()
            .map(|product| Product {
                id: Some(product.get::<i64, _>("id") as u64),
                name: product.get::<String, _>("name"),
                stock: product.get::<i32, _>("stock") as u32,
            })
            .collect())
    }
    async fn get_by_id(&self, id: u64) -> Result<Product> {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        let product = sqlx::query("SELECT * FROM products WHERE id=?")
            .bind(id as i64)
            .fetch_one(&(*pool))
            .await
            .map_err(Error::map("get by id".into()))?;
        Ok(Product {
            id: Some(id),
            name: product.get::<String, _>("name"),
            stock: product.get::<i32, _>("stock") as u32,
        })
    }
    async fn save(&self, mut product: Product) -> Result<Product> {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        product.id = Some(
            match product.id {
                Some(id) => {
                    let update_product = product.clone();
                    sqlx::query("UPDATE products SET name=?, stock=? WHERE id=?")
                        .bind(update_product.name)
                        .bind(update_product.stock)
                        .bind(id as i64)
                        .execute(&(*pool))
                        .await
                        .map_err(|err| -> Error {
                            Error {
                                cause: err.to_string(),
                                currently_on: "product update".into(),
                            }
                        })
                }
                None => {
                    let new_product = product.clone();
                    sqlx::query("INSERT INTO products(name, stock) VALUES(?,?)")
                        .bind(new_product.name)
                        .bind(new_product.stock)
                        .execute(&(*pool))
                        .await
                        .map_err(|err| -> Error {
                            Error {
                                cause: err.to_string(),
                                currently_on: "product insert".into(),
                            }
                        })
                }
            }?
            .last_insert_rowid() as u64,
        );
        drop(pool);
        Ok(product)
    }
    async fn delete(&self, id: u64) -> Result<bool> {
        let pool = self.pool.clone();
        let pool = pool.lock().await;
        sqlx::query("DELETE FROM products WHERE id=?")
            .bind(id as i64)
            .execute(&(*pool))
            .await
            .map_err(|err| -> Error {
                Error {
                    cause: err.to_string(),
                    currently_on: "product update".into(),
                }
            })?;
        drop(pool);
        Ok(true)
    }
}
