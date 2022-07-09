use std::time::Duration;

use rocket::{
    get,
    serde::json::Json,
    tokio::{sync::RwLock, time},
    State,
};

use crate::{models::home::HomeModel, response::Response};

#[get("/home", format = "json")]
pub async fn handler(home_model: &State<RwLock<HomeModel>>) -> Json<Response<HomeModel>> {
    time::sleep(Duration::from_secs(1)).await;
    Json(Response {
        status: true,
        message: String::new(),
        data: Some(home_model.read().await.clone()),
    })
}
