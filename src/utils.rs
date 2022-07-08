use std::path::Path;

use rocket::{
    serde::{DeserializeOwned, Serialize},
    tokio::{
        fs::File,
        io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    },
};

pub async fn write_to_json<T: Serialize>(filepath: impl AsRef<Path>, data: &T) -> io::Result<()> {
    let file = File::create(filepath).await?;
    let mut writer = BufWriter::<File>::new(file);
    writer
        .write(serde_json::to_string_pretty(data)?.as_bytes())
        .await?;
    writer.flush().await?;
    Ok(())
}

pub async fn read_from_json<T: DeserializeOwned>(filepath: impl AsRef<Path>) -> Option<T> {
    let file = File::open(filepath).await.ok()?;
    let mut reader = BufReader::new(file);
    let mut json = String::new();
    reader.read_to_string(&mut json).await.ok()?;
    let data = serde_json::from_str(&json).ok()?;
    reader.flush().await.ok()?;
    Some(data)
}
