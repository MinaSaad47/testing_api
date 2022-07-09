use crate::utils::IJson;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct CategoriesModel {
    #[serde(default)]
    pub current_page: u32,
    #[serde(default)]
    pub data: Vec<DataModel>,
}

impl IJson for CategoriesModel {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct DataModel {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub image: String,
}
