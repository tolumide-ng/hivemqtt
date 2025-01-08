use std::future::Future;

use futures::AsyncReadExt;

use crate::v5::commons::error::MQTTError;

pub(crate) trait AsyncRead<S>: Sized {
    fn async_read(stream: &mut S) -> impl Future<Output = Result<Self, MQTTError>>;
}

impl<S> AsyncRead<S> for u8
    where S: AsyncReadExt + Unpin {
        async fn async_read(stream: &mut S) -> Result<u8, MQTTError> {
            let mut buf = vec![0u8; std::mem::size_of::<u8>()];
            stream.read_exact(&mut buf).await?;
            
            Ok(u8::from_le_bytes(buf.try_into().unwrap()))
        }
}

impl<S> AsyncRead<S> for u16 
    where S: AsyncReadExt + Unpin {
        async fn async_read(stream: &mut S) -> Result<Self, MQTTError> {
            let mut buf = vec![0u8; std::mem::size_of::<u16>()];
            stream.read_exact(&mut buf).await?;
            
            Ok(u16::from_le_bytes(buf.try_into().unwrap()))
        }
}

impl<S> AsyncRead<S> for u32 
    where S: AsyncReadExt + Unpin {
        async fn async_read(stream: &mut S) -> Result<Self, MQTTError> {
            let mut buf = vec![0u8; std::mem::size_of::<u32>()];
            stream.read_exact(&mut buf).await?;
            
            Ok(u32::from_le_bytes(buf.try_into().unwrap()))
        }
}


impl<S> AsyncRead<S> for String 
    where S: AsyncReadExt + Unpin {
        async fn async_read(stream: &mut S) -> Result<Self, MQTTError> {
            let len = u8::async_read(stream).await?;
            let mut buf = vec![0u8; len as usize];
            stream.read_exact(&mut buf).await?;

            let value = String::from_utf8(buf).map_err(|e| MQTTError::Utf8Error(e))?;
            Ok(value)
        }
}