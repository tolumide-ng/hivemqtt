use std::future::Future;

use bytes::Bytes;
use futures::AsyncWriteExt;

use crate::v5::commons::error::MQTTError;

pub(crate) trait AsyncWrite<S>: Sized {
    fn write(&self, stream: &mut S) -> impl Future<Output = Result<(), MQTTError>>;
}

impl<S> AsyncWrite<S> for u8
    where S: AsyncWriteExt + Unpin {
        async fn write(&self, stream: &mut S) -> Result<(), MQTTError> {
            stream.write_all(&self.to_be_bytes()).await?;
            Ok(())
        }
}

impl<S> AsyncWrite<S> for u16
    where S: AsyncWriteExt + Unpin {
        async fn write(&self, stream: &mut S) -> Result<(), MQTTError> {
            stream.write_all(&self.to_be_bytes()).await?;
            Ok(())
        }
}

impl<S> AsyncWrite<S> for u32
    where S: AsyncWriteExt + Unpin {
        async fn write(&self, stream: &mut S) -> Result<(), MQTTError> {
            stream.write_all(&self.to_be_bytes()).await?;
            Ok(())
        }
}

impl<S> AsyncWrite<S> for Bytes
    where S: AsyncWriteExt + Unpin {
        async fn write(&self, stream: &mut S) -> Result<(), MQTTError> {
            stream.write_all(&(self.len() as u16).to_be_bytes()).await?;
            stream.write_all(&self).await?;
            Ok(())
        }
}

impl<S> AsyncWrite<S> for String
    where S: AsyncWriteExt + Unpin {
        async fn write(&self, stream: &mut S) -> Result<(), MQTTError> {
            stream.write_all(&(self.len() as u16).to_be_bytes()).await?;
            stream.write_all(&self.as_bytes()).await?;
            Ok(())
        }
}