use crate::models::{product::Product, AppState};
use rocket::{response::Responder, serde::json::Json, State};

#[derive(Responder)]
pub enum Response {
    #[response(status = 200)]
    Success(Json<Vec<Product>>),
    #[response(status = 500)]
    InternalServerError(String),
}

#[get("/")]
pub async fn index(state: &State<AppState>) -> Response {
    match state.product_repo.lock().await.get_all().await {
        Ok(result) => Response::Success(Json::from(result)),
        Err(err) => Response::InternalServerError(err.cause),
    }
}
