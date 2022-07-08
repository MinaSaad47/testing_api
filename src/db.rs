use rocket::{
    serde::{Deserialize, DeserializeOwned, Serialize},
    tokio::io,
};

use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    path::Path,
};

use crate::utils;

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

impl<K, V> DB<K, V>
where
    K: DeserializeOwned + Serialize + Hash + Eq,
    V: DeserializeOwned + Serialize,
{
    pub async fn to_json(&self, filepath: impl AsRef<Path>) -> io::Result<()> {
        utils::write_to_json(filepath, self).await
    }
    pub async fn from_json(filepath: impl AsRef<Path>) -> Option<Self> {
        utils::read_from_json(filepath).await
    }
}
