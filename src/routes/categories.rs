use crate::{models::categories::CategoriesModel, response::Response};
use rocket::{
    get,
    serde::json::Json,
    tokio::{sync::RwLock, time},
    State,
};
use std::time::Duration;

#[get("/categories", format = "json")]
pub async fn handler(
    categories_model: &State<RwLock<CategoriesModel>>,
) -> Json<Response<CategoriesModel>> {
    time::sleep(Duration::from_secs(1)).await;
    Json(Response {
        status: true,
        message: String::new(),
        data: Some(categories_model.read().await.clone()),
    })
}
