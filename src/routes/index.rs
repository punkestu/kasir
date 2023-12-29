use crate::models::{product::Product, AppState};
use rocket::{
    response::{content::RawHtml, Responder},
    State,
};
use serde::Serialize;
use tera::Context;

#[derive(Responder)]
pub enum Response {
    #[response(status = 200)]
    Success(RawHtml<String>),
    #[response(status = 500)]
    InternalServerError(String),
}

#[derive(Serialize, Default)]
struct IndexViewParam {
    products: Vec<Product>,
}

#[get("/")]
pub async fn index(state: &State<AppState>) -> Response {
    match state.product_repo.lock().await.get_all().await {
        Ok(products) => {
            let index_view_param = IndexViewParam { products };
            let view = state
                .tera
                .lock()
                .await
                .render(
                    "pages/home.html",
                    &Context::from_serialize(&index_view_param).unwrap(),
                )
                .unwrap();
            Response::Success(RawHtml(view))
        }
        Err(err) => Response::InternalServerError(err.cause),
    }
}
