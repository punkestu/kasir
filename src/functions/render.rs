use serde::Serialize;
use tera::{Context, Tera};

use crate::models::{product::Product, Error};

#[derive(Serialize)]
pub struct View<P> {
    param: P,
    title: &'static str,
    page: &'static str,
}

fn render_bootstrap<'a, P>(view: View<P>) -> impl FnOnce(&Tera) -> tera::Result<String> + 'a
where
    P: Serialize + 'a,
{
    move |tera: &Tera| -> tera::Result<String> {
        tera.render(
            format!("pages/{}", view.page).as_str(),
            &Context::from_serialize(view).unwrap(),
        )
    }
}

#[derive(Serialize, Default)]
pub struct HomeParam {
    pub products: Vec<Product>,
}

pub fn render_home(tera: &Tera) -> impl Fn(HomeParam) -> Result<String, Error> + '_ {
    move |param: HomeParam| -> Result<String, Error> {
        render_bootstrap(View {
            param,
            title: "Kasir",
            page: "home.tera",
        })(tera)
        .map_err(Error::map("render home".into()))
    }
}

pub fn render_product_partials(
    tera: &Tera,
) -> impl FnOnce(Vec<Product>) -> Result<String, Error> + '_ {
    move |products: Vec<Product>| -> Result<String, Error> {
        tera.render(
            "partials/products.tera",
            &Context::from_serialize(HomeParam { products }).unwrap(),
        )
        .map_err(Error::map("render home".into()))
    }
}

#[derive(Serialize)]
pub struct CartParam {
    pub items: Vec<u32>,
}

pub fn render_cart(tera: &Tera) -> impl Fn(&CartParam) -> Result<String, Error> + '_ {
    move |param: &CartParam| -> Result<String, Error> {
        render_bootstrap(View {
            param,
            title: "Kasir",
            page: "cart.tera",
        })(tera)
        .map_err(Error::map("render cart".into()))
    }
}
