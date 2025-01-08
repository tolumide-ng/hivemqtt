// use std::string::FromUtf8Error;

// use futures::AsyncReadExt;

// use crate::v5::traits::asyncx::le_bytes::FromLeBytes;

// pub(crate) trait ReadStreamExt: AsyncReadExt + Unpin {
//     /// Reads a fixed-size value from the stream (e.g., u8, u16, u32).
//     async fn read<T>(&mut self) -> std::io::Result<T>
//         where T: FromLeBytes {
//             let mut buf = vec![0u8; std::mem::size_of::<T>()];
//             self.read_exact(&mut buf).await?;
//             Ok(T::from_le_bytes(&buf))
//     }

//     async fn read_str(&mut self) -> std::io::Result<Result<String, FromUtf8Error>> {
//         let len = ReadStreamExt::read::<u16>(self).await?;
//         let mut buf = vec![0u8; len as usize];
//         self.read_exact(&mut buf).await?;

//         Ok(String::from_utf8(buf))
//     }

//     async fn read_vec(&mut self) -> std::io::Result<Option<Vec<u16>>> {
//         let has_value = ReadStreamExt::read::<u8>(self).await?;
//         if has_value == 0 { return Ok(None) }

//         let len = ReadStreamExt::read::<u16>(self).await?;
//         let mut buf = vec![0u8; len as usize];
//         self.read_exact(&mut buf).await?;
        
//         let result = buf.chunks(2).map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap())).collect::<Vec<u16>>();
//         Ok(Some(result))
//     }
// }
