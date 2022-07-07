use rocket::{get, serde::json::Json, tokio::sync::RwLock, State};

use crate::{models::home::HomeModel, response::Response};

#[get("/home", format = "json")]
pub async fn handler(home_model: &State<RwLock<HomeModel>>) -> Json<Response<HomeModel>> {
    Json(Response {
        status: true,
        message: String::new(),
        data: Some(home_model.read().await.clone()),
    })
}
