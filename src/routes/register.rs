use rocket::{post, response::status::Created, serde::json::Json, tokio::sync::RwLock, uri, State};

use crate::{db::DB, models::login::LoginModel, routes, utils::IJson};

#[post("/register", format = "json", data = "<login_model>")]
pub async fn handler(
    login_model: Json<LoginModel>,
    db: &State<RwLock<DB<String, LoginModel>>>,
) -> Created<Json<LoginModel>> {
    let mut login_model = login_model.into_inner();

    login_model.id = db.read().await.len() as u32;

    let inserted_login_model = login_model.clone();

    db.write().await.insert(
        format!("{}:{}", login_model.email, login_model.password),
        inserted_login_model,
    );

    db.read()
        .await
        .write_to_json("login_model.json")
        .await
        .unwrap();

    let location = uri!("/api", routes::login::handler());
    Created::new(location.to_string()).body(Json(login_model))
}
