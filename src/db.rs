use crate::utils::IJson;
use rocket::serde::{Deserialize, DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct DB<K: Hash + Eq, V>(HashMap<K, V>);

impl<K, V> DB<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K, V> Deref for DB<K, V>
where
    K: Hash + Eq,
{
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for DB<K, V>
where
    K: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> IJson for DB<K, V>
where
    K: DeserializeOwned + Serialize + Hash + Eq,
    V: DeserializeOwned + Serialize,
{
}
