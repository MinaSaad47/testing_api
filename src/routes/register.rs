use rocket::{post, response::status::Created, serde::json::Json, tokio::sync::RwLock, uri, State};

use crate::{db::DB, models::login::UserModel, response::Response, routes, utils::IJson};

#[post("/register", format = "json", data = "<login_model>")]
pub async fn handler(
    login_model: Json<UserModel>,
    db: &State<RwLock<DB<String, UserModel>>>,
    authorization: &State<RwLock<DB<String, String>>>,
) -> Created<Json<Response<UserModel>>> {
    let mut login_model = login_model.into_inner();

    let key = format!("{}:{}", login_model.email, login_model.password);
    let token = base64::encode(&key);

    login_model.id = db.read().await.len() as u32;
    login_model.token = token.clone();

    let inserted_login_model = login_model.clone();

    authorization.write().await.insert(token, key.clone());
    db.write().await.insert(key, inserted_login_model);

    db.read()
        .await
        .write_to_json("login_model.json")
        .await
        .unwrap();

    let location = uri!("/api", routes::login::handler());
    Created::new(location.to_string()).body(Json(Response {
        status: true,
        message: "Created account successfully".to_string(),
        data: Some(login_model),
    }))
}
