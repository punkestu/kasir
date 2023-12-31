use super::{InternalServerError, Result};
use crate::{
    functions::render::{self, CartParam, HomeParam},
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
}

#[get("/")]
pub async fn index(state: &State<AppState>) -> Result<impl Responder> {
    match state.product_repo.lock().await.get_all().await {
        Ok(products) => {
            let param = HomeParam { products };
            let view = Result::<HomeParam>::Ok(param)
                .map(render::render_home(&state.tera))?
                .map_err(InternalServerError::from)?;
            Ok(Response::Success(RawHtml(view)))
        }
        Err(err) => Err(InternalServerError(err.cause)),
    }
}

#[get("/cart")]
pub async fn cart(state: &State<AppState>) -> Result<impl Responder> {
    let param = CartParam { items: vec![] };
    let view = Result::<CartParam>::Ok(param)
        .map(render::render_cart(&state.tera))?
        .map_err(InternalServerError::from)?;
    Ok(Response::Success(RawHtml(view)))
}
