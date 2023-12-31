use crate::models::product::Product;

pub fn product_with_name_like(like: &str) -> impl FnMut(&Product) -> bool + '_ {
    move |product: &Product| -> bool { product.name.contains(like) }
}
