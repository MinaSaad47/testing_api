use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct LoginModel {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub phone: String,
    pub password: String,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub points: i32,
    #[serde(default)]
    pub credit: i32,
    #[serde(default)]
    pub token: String,
}
