use std::io::Result;

use async_trait::async_trait;

#[async_trait]
pub trait AsyncStream {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.read(buf).await
    }

    async fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write(buf).await
    }

    async fn flush(&mut self) -> Result<()>;
}
