use crate::{
    functions::{self, render},
    models::AppState,
};
use rocket::{
    response::{content::RawHtml, Responder},
    State,
};

#[derive(Responder)]
pub enum Response {
    #[response(status = 200)]
    Success(RawHtml<String>),
    #[response(status = 500)]
    InternalServerError(String),
}

#[get("/")]
pub async fn index(state: &State<AppState>) -> Response {
    match state.product_repo.lock().await.get_all().await {
        Ok(products) => {
            let param = functions::render::HomeParam { products };
            let view = Some(&state.tera.clone())
                .map(render::render_home(param))
                .unwrap();
            Response::Success(RawHtml(view))
        }
        Err(err) => Response::InternalServerError(err.cause),
    }
}
