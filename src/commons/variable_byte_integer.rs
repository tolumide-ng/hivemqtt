use bytes::{BufMut, Bytes, BytesMut};

use super::error::MQTTError;

pub trait VariableByteInteger {
    fn encode(buff: &mut BytesMut, len: usize) -> Result<(), MQTTError> {
        // 268_435_455
        if len > 0xFFFFFFF {
            return Err(MQTTError::PayloadTooLong)
        }
        let mut x = len;

        while x > 0 {
            // Get the last 7bits of x;
            let mut encoded_bytes = (x & 0x7F) as u8; // (x%128)
            x >>= 7; // shift to right by 7bits (i.e x = x/128)

            if x > 0 {
                encoded_bytes |= 0x80
            }
            buff.put_u8(encoded_bytes);
        }

        Ok(())
    }

    fn decode() {}
}