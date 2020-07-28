use std::path::Path;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io;

use async_trait::async_trait;

use crate::{Result, Storager};

struct Storage {
    work_dir: Path,
}

#[async_trait]
impl Storager for Storage {
    async fn stat(&self, path: &Path) {}
    async fn delete(&self, path: &Path) -> Result<()> {
        // TODO: We need to handle os separator.
        let rp = self.work_dir.join(path);

        Ok(fs::remove_file(rp).await?)
    }
    async fn read(&self, path: &Path) -> Result<Box<dyn tokio::io::AsyncRead + Unpin>> {
        let rp = self.work_dir.join(path);
        let file = tokio::fs::File::open(rp).await?;

        Ok(Box::new(file))
    }
    async fn write<R: io::AsyncRead + Unpin + Send + Sync>(
        &self,
        path: &Path,
        r: &mut R,
    ) -> Result<()> {
        let rp = self.work_dir.join(path);
        let mut file = OpenOptions::new().create(true).write(true).open(rp).await?;

        io::copy(r, &mut file).await?;

        Ok(())
    }
}
