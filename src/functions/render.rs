use std::sync::Arc;

use serde::Serialize;
use tera::{Context, Tera};

use crate::models::product::Product;

#[derive(Serialize, Default)]
pub struct HomeParam {
    pub products: Vec<Product>,
}

// pub fn render_home(tera: &Tera, param: HomeParam) -> String {
//     tera.render("pages/home.tera", &Context::from_serialize(param).unwrap())
//         .unwrap()
// }

pub fn render_home(param: HomeParam) -> impl Fn(&Arc<Tera>) -> String {
    move |tera: &Arc<Tera>| -> String {
        tera.render("pages/home.tera", &Context::from_serialize(&param).unwrap())
            .unwrap()
    }
}
