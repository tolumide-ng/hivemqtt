use bytes::{Bytes, BytesMut};
use error::MQTTError;

pub mod packets;
pub mod property;
pub mod reason_code;
pub mod qos;


pub(crate) mod variable_byte_integer;
pub(crate) mod error;
pub(crate) mod version; // good
pub(crate) mod fixed_header;

use crate::traits::{write::Write, read::Read};

/// Encodes the integer into a variable byte integer
fn encode_vbi(buf: &mut BytesMut, len: usize) -> Result<(), MQTTError> {
    let mut len = len;

        // 268_435_455
    if len > 0xFFFFFFF { return Err(MQTTError::PayloadTooLong) }

    for _ in 0..4 {
        let mut byte= len % 128;
        len /= 128;

        if len > 0 { byte |= 128; }
        
        (byte as u8).write(buf); // writes the encoded byte into the buffer
        if len == 0 { break; }
    }
    Ok(())
}



 /// Decodes a Variable byte Inetger
fn decode(buf: &mut Bytes) -> Result<usize, MQTTError> {
    let mut result = 0;

    for i in 0..4 {
        if buf.is_empty() {
            return Err(MQTTError::MalformedPacket);
        }
        let byte = u8::read(buf)?;

        result += ((byte as usize) & 0x7F) << (7 * i);

        if (byte & 0x80) == 0 {
            return Ok(result)
        }
    }

    
    return Err(MQTTError::MalformedPacket)
}