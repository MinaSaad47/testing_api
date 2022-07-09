use std::path::Path;

use rocket::{
    async_trait,
    serde::{DeserializeOwned, Serialize},
    tokio::{
        fs::File,
        io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
    },
};

#[async_trait]
pub trait IJson: Serialize + DeserializeOwned {
    async fn write_to_json(&self, filepath: impl AsRef<Path> + Send) -> io::Result<()> {
        let file = File::create(filepath).await?;
        let mut writer = BufWriter::<File>::new(file);
        writer
            .write(serde_json::to_string_pretty(self)?.as_bytes())
            .await?;
        writer.flush().await?;
        Ok(())
    }

    async fn read_from_json(filepath: impl AsRef<Path> + Send) -> Option<Self> {
        let file = File::open(filepath).await.ok()?;
        let mut reader = BufReader::new(file);
        let mut json = String::new();
        reader.read_to_string(&mut json).await.ok()?;
        let data = serde_json::from_str(&json).ok()?;
        reader.flush().await.ok()?;
        Some(data)
    }
}
