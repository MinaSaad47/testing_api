#![allow(dead_code)]

use std::{
    time::Duration,
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
};

use rocket::{
    response::status::Created,
    serde::{json::Json, Deserialize, Serialize},
    tokio::{sync::RwLock, time},
    State,
};

#[macro_use]
extern crate rocket;

type DataBase = RwLock<HashMap<(Arc<String>, Arc<String>), User>>;
type IDCounter = AtomicU32;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
struct User {
    #[serde(default)]
    id: u32,
    #[serde(default)]
    name: String,
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    age: u32,
    password: Arc<String>,
    email: Arc<String>,
}

#[post("/register", format = "json", data = "<user>")]
async fn register_handler(
    user: Json<User>,
    new_id: &State<IDCounter>,
    db: &State<DataBase>,
) -> Created<Json<User>> {
    let id = new_id.fetch_add(1, Ordering::Relaxed);
    let user = user.into_inner();
    let user = User {
        id,
        name: user.name,
        password: user.password,
        email: user.email,
        ..Default::default()
    };

    let inserted_user = user.clone();

    db.write().await.insert(
        (
            Arc::clone(&inserted_user.email),
            Arc::clone(&inserted_user.password),
        ),
        inserted_user,
    );

    let location = uri!("/api", login_handler());
    Created::new(location.to_string()).body(Json(user))
}

#[post("/login", format = "json", data = "<user>")]
async fn login_handler(user: Json<User>, db: &State<DataBase>) -> Option<Json<User>> {
    time::sleep(Duration::from_secs(3)).await; // simulate remote server.
    let user = user.into_inner();
    db.read()
        .await
        .get(&(user.email, user.password))
        .map(|v| Json(v.clone()))
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/api", routes![register_handler, login_handler])
        .manage(DataBase::new(HashMap::new()))
        .manage(IDCounter::new(0))
}
