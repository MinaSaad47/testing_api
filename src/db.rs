use rocket::{
    serde::{Deserialize, DeserializeOwned, Serialize},
    tokio::{
        fs::File,
        io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    },
};

use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    path::Path,
};

#[derive(Debug, Serialize, Deserialize)]
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
        let file = File::create(filepath).await?;
        let mut writer = BufWriter::<File>::new(file);
        writer
            .write(serde_json::to_string_pretty(self)?.as_bytes())
            .await?;
        Ok(())
    }
    pub async fn from_json(filepath: impl AsRef<Path>) -> Option<Self> {
        let file = File::open(filepath).await.ok()?;
        let mut reader = BufReader::new(file);
        let mut json = String::new();
        reader.read_to_string(&mut json).await.ok()?;
        serde_json::from_str(&json).ok()?
    }
}
