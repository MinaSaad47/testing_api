use crate::utils::IJson;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Banner {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Number(serde_json::Number);

impl Number {
    pub fn from_i32(v: i32) -> Self {
        Self(serde_json::Number::from(v))
    }
    pub fn from_f32(v: f32) -> Self {
        Self(serde_json::Number::from_f64(v.into()).unwrap())
    }
}

impl Default for Number {
    fn default() -> Self {
        Self(serde_json::Number::from(0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    pub price: Number,
    pub old_price: Number,
    pub discount: Number,
    #[serde(default)]
    pub image: String,
    #[serde(default)]
    pub in_favorites: bool,
    #[serde(default)]
    pub in_cart: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct HomeModel {
    pub banners: Vec<Banner>,
    pub products: Vec<Product>,
}

impl IJson for HomeModel {}
