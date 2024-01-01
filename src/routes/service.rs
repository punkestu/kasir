use super::{InternalServerError, Result};
use crate::models::{product::Product, AppState};
use rocket::{
    response::{content::RawHtml, Responder},
    serde::{json::Json, Deserialize},
    State,
};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct SetCartItemParam {
    pub item_id: u64,
    pub qty: u32,
}

#[derive(Responder)]
pub enum SetCartItemResponse {
    #[response(status = 200)]
    Success(RawHtml<String>),
}

#[put("/item", data = "<item>")]
pub async fn set_cart_item(
    state: &State<AppState>,
    item: Json<SetCartItemParam>,
) -> Result<impl Responder> {
    let mut cart_items = state.cart_buf.lock().await;
    match state
        .product_repo
        .lock()
        .await
        .get_by_id(item.item_id)
        .await
    {
        Ok(product) => {
            cart_items.push(Product {
                stock: item.qty,
                ..product
            });
            Ok(SetCartItemResponse::Success(RawHtml("Ok".into())))
        }
        Err(product_err) => Err(InternalServerError::from(product_err.cause)),
    }
}

#[derive(Responder)]
#[response(status = 200)]
pub struct GetCartItemsSuccess(Json<Vec<Product>>);

#[get("/items")]
pub async fn get_cart_items(state: &State<AppState>) -> Result<impl Responder> {
    let cart_items = state.cart_buf.lock().await;
    Ok(GetCartItemsSuccess(Json(cart_items.to_vec())))
}
