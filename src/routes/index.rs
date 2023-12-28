use crate::models::{product::Product, AppState};
use rocket::{serde::json::Json, State};

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct IndexSuccess(Json<Vec<Product>>);
#[get("/")]
pub async fn index(state: &State<AppState>) -> IndexSuccess {
    IndexSuccess(Json::from(state.product_repo.lock().await.get_all().await))
}
