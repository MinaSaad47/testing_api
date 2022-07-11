use std::time::Duration;

use rocket::{
    post,
    serde::json::Json,
    serde::{Deserialize, Serialize},
    tokio::{sync::RwLock, time},
    State,
};

use crate::{
    models::{
        home::{HomeModel, Product},
        login::UserModel,
    },
    response::Response,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct SearchTerm {
    pub text: String,
}

#[post("/product/search", format = "json", data = "<term>")]
pub async fn handler(
    term: Json<SearchTerm>,
    home_model: &State<RwLock<HomeModel>>,
    _user: UserModel,
) -> Json<Response<Vec<Product>>> {
    time::sleep(Duration::from_secs(1)).await;
    let text = term.into_inner().text;

    let data: Vec<Product> = home_model
        .read()
        .await
        .products
        .iter()
        .filter(|product| product.name.contains(&text))
        .cloned()
        .collect();

    Json(Response {
        status: true,
        message: String::new(),
        data: Some(data),
    })
}
