#![allow(dead_code)]

use rocket::tokio::sync::RwLock;
use testing_api::{
    db::DB,
    models::{home::HomeModel, login::LoginModel},
    routes,
};

#[macro_use]
extern crate rocket;

#[launch]
async fn launch() -> _ {
    let login_model = DB::<String, LoginModel>::from_json("login_model.json")
        .await
        .unwrap();
    let home_model = HomeModel::from_json("home_model.json").await.unwrap();
    rocket::build()
        .mount(
            "/api",
            routes![
                routes::register::handler,
                routes::login::handler,
                routes::home::handler,
            ],
        )
        .manage(RwLock::new(login_model))
        .manage(RwLock::new(home_model))
}
