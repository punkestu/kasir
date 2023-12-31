use super::{InternalServerError, Result};
use crate::{
    functions::{filter::product_with_name_like, render::render_product_partials},
    models::{product::Product, AppState},
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

#[derive(FromForm)]
pub struct ProductsQuery {
    pub search: Option<String>,
}

#[get("/products?<query..>")]
pub async fn products(state: &State<AppState>, query: ProductsQuery) -> Result<Response> {
    let products = state
        .product_repo
        .clone()
        .lock()
        .await
        .get_all()
        .await
        .map_err(InternalServerError::from)?;
    match query.search {
        Some(search) => {
            let products: Vec<Product> = products
                .into_iter()
                .filter(product_with_name_like(&search))
                .collect();
            let view = Result::<Vec<Product>>::Ok(products)
                .map(render_product_partials(&state.tera))?
                .map_err(InternalServerError::from)?;
            Ok(Response::Success(RawHtml(view)))
        }
        None => Ok(Response::Success(RawHtml("none".into()))),
    }
}
