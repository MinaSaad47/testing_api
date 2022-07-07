use rocket::{post, response::status::Created, serde::json::Json, tokio::sync::RwLock, uri, State};

use crate::{db::DB, models::login::LoginModel, routes};

#[post("/register", format = "json", data = "<login_model>")]
pub async fn handler(
    login_model: Json<LoginModel>,
    db: &State<RwLock<DB<String, LoginModel>>>,
) -> Created<Json<LoginModel>> {
    let login_model = login_model.into_inner();
    let login_model = LoginModel {
        name: login_model.name,
        password: login_model.password,
        email: login_model.email,
        ..Default::default()
    };

    let inserted_login_model = login_model.clone();

    db.write().await.insert(
        format!("{}:{}", login_model.email, login_model.password,).clone(),
        inserted_login_model,
    );

    db.read().await.to_json("login_model.json").await.unwrap();

    let location = uri!("/api", routes::login::handler());
    Created::new(location.to_string()).body(Json(login_model))
}
