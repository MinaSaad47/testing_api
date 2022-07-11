use rocket::{
    post,
    serde::json::Json,
    tokio::{sync::RwLock, time},
    State,
};

use std::time::Duration;

use crate::{db::DB, models::login::UserModel, response::Response};

#[post("/login", format = "application/json", data = "<login_model>")]
pub async fn handler(
    login_model: Json<UserModel>,
    db: &State<RwLock<DB<String, UserModel>>>,
    authorization: &State<RwLock<DB<String, String>>>,
) -> Json<Response<UserModel>> {
    let mut authorization = authorization.inner().write().await;
    time::sleep(Duration::from_secs(1)).await; // simulate remote server.
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
                |model| {
                    let key = format!("{}:{}", model.email, model.password);
                    authorization.insert(base64::encode(&key), key);
                    Response {
                        data: Some(model.clone()),
                        status: true,
                        message: "Logged in successfully".to_string(),
                    }
                },
            ),
    )
}
