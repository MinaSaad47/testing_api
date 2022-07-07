#![allow(dead_code)]

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
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
    email: Arc<String>,
    #[serde(default)]
    phone: String,
    password: Arc<String>,
    #[serde(default)]
    image: String,
    #[serde(default)]
    points: i32,
    #[serde(default)]
    credit: i32,
    #[serde(default)]
    token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
struct Response {
    status: bool,
    message: String,
    data: Option<User>,
}

impl Response {
    async fn from_database(db: &DataBase, email: Arc<String>, password: Arc<String>) -> Self {
        db.read().await.get(&(email, password)).map_or_else(
            || Response {
                status: false,
                message: "Could not log in, please be certain of the entered data".to_string(),
                data: None,
            },
            |user| Response {
                status: true,
                message: "logged in successfully".to_string(),
                data: Some(user.clone()),
            },
        )
    }
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
async fn login_handler(user: Json<User>, db: &State<DataBase>) -> Json<Response> {
    time::sleep(Duration::from_secs(1)).await; // simulate remote server.
    let user = user.into_inner();
    Json(Response::from_database(db, user.email, user.password).await)
}

#[launch]
fn launch() -> _ {
    rocket::build()
        .mount("/api", routes![register_handler, login_handler])
        .manage(DataBase::new(HashMap::new()))
        .manage(IDCounter::new(0))
}
