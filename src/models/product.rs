use rocket::serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Product {
    pub id: Option<u64>,
    pub name: String,
    pub stock: u32,
}

#[async_trait::async_trait]
pub trait ProductRepo {
    async fn get_all(&self) -> Vec<Product>;
    async fn save(&self, product: Product) -> Product;
    async fn delete(&self, id: u64) -> bool;
}
