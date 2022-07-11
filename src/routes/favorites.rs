use crate::{
    models::{home::{HomeModel, Product}, login::UserModel},
    response::Response,
    utils::IJson,
};
use rocket::{
    http::Status,
    put,
    serde::{json::Json, Deserialize, Serialize},
    tokio::{sync::RwLock, time},
    State,
};

use std::{ops::Deref, time::Duration};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ProductId {
    pub product_id: u32,
}

impl Deref for ProductId {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.product_id
    }
}

#[put("/favorites", format = "json", data = "<product_id>")]
pub async fn handler(
    product_id: Json<ProductId>,
    home_model: &State<RwLock<HomeModel>>,
    _user: UserModel,
) -> (Status, Json<Response<Product>>) {
    time::sleep(Duration::from_secs(1)).await;
    let product_id = product_id.into_inner();
    let mut home_model = home_model.inner().write().await;
    let product = home_model
        .products
        .iter_mut()
        .find(|product| product.id == *product_id);
    match product {
        Some(product) => {
            product.in_favorites = !product.in_favorites;
            let product = product.clone();
            home_model.write_to_json("home_model.json").await.unwrap();
            (
                Status::Ok,
                Json(Response {
                    status: true,
                    message: "Product updated".to_string(),
                    data: Some(product),
                }),
            )
        }
        None => (
            Status::NoContent,
            Json(Response {
                status: false,
                message: "Product not found".to_string(),
                data: None,
            }),
        ),
    }
}
