use rocket::{
    post,
    serde::json::Json,
    tokio::{sync::RwLock, time},
    State,
};

use std::time::Duration;

use crate::{db::DB, models::login::LoginModel, response::Response};

#[post("/login", format = "json", data = "<login_model>")]
pub async fn handler(
    login_model: Json<LoginModel>,
    db: &State<RwLock<DB<String, LoginModel>>>,
) -> Json<Response<LoginModel>> {
    time::sleep(Duration::from_secs(1)).await; // simulate remote server.
    let login_model = login_model.into_inner();
    Json(
        db.read()
            .await
            .get(&format!("{}:{}", login_model.email, login_model.password))
            .map_or_else(
                || Response {
                    data: None,
                    status: false,
                    message: "could not log in, please be sure about the provided info".to_string(),
                },
                |model| Response {
                    data: Some(model.clone()),
                    status: true,
                    message: "Logged in successfully".to_string(),
                },
            ),
    )
}
