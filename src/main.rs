#![allow(dead_code)]

use rocket::tokio::sync::RwLock;
use testing_api::{
    db::DB,
    routes,
    utils::IJson,
};

#[macro_use]
extern crate rocket;

#[launch]
async fn launch() -> _ {
    let login_model = DB::<String, LoginModel>::read_from_json("login_model.json")
        .await
        .unwrap_or_default();
    let home_model = HomeModel::read_from_json("home_model.json")
        .await
        .unwrap_or_default();
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
