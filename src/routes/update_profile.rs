use crate::{db::DB, models::login::UserModel, response::Response, routes, utils::IJson};
use rocket::{
    put,
    response::status::Created,
    serde::json::Json,
    tokio::{sync::RwLock, time},
    uri, State,
};
use std::time::Duration;

#[put("/update-profile", format = "json", data = "<profile_update>")]
pub async fn handler(
    profile_update: Json<UserModel>,
    db: &State<RwLock<DB<String, UserModel>>>,
    authorization: &State<RwLock<DB<String, String>>>,
    user_model: UserModel,
) -> Created<Json<Response<UserModel>>> {
    time::sleep(Duration::from_secs(2)).await; // simulate remote server.
    let profile_update = profile_update.into_inner();
    let authorization = authorization.inner();
    let db = db.inner();

    let (name, email, password) = (
        if profile_update.name != String::default() {
            profile_update.name
        } else {
            user_model.name
        },
        if profile_update.email != String::default() {
            profile_update.email
        } else {
            user_model.email
        },
        if profile_update.password != String::default() {
            profile_update.password
        } else {
            user_model.password
        },
    );

    let key = format!("{}:{}", email, password);
    let token = base64::encode(&key);

    let old_key = authorization
        .write()
        .await
        .remove(&user_model.token)
        .expect("must be valid since the user is already authorized");

    let old_user = db
        .write()
        .await
        .remove(&old_key)
        .expect("must be valid since the user is already authorized");

    let new_user = UserModel {
        name,
        email,
        password,
        token: token.clone(),
        ..old_user
    };

    authorization.write().await.insert(token, key.clone());
    db.write().await.insert(key, new_user.clone());

    db.read()
        .await
        .write_to_json("login_model.json")
        .await
        .unwrap();

    let location = uri!("/api", routes::login::handler());
    Created::new(location.to_string()).body(Json(Response {
        status: true,
        message: "Updated account successfully".to_string(),
        data: Some(new_user),
    }))
}
