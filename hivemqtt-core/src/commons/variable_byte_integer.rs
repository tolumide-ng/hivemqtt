use bytes::{BufMut, Bytes, BytesMut};

use crate::commons::error::MQTTError;


/// The Variable Byte Integer(VBI) is encoded using an encoding scheme which uses a single byte for values up to 127
pub(crate) trait VariableByteInteger {
    fn encode(buff: &mut BytesMut, len: usize) -> Result<(), MQTTError> {
        // 268_435_455
        if len > 0xFFFFFFF {
            return Err(MQTTError::PayloadTooLong)
        }
        let mut x = len;

        while x > 0 {
            // Get the last 7bits of x;
            let mut encoded_bytes = (x & 0x7F) as u8; // (x%128)
            x >>= 7; // shift to right by 7bits(i.e. 2^7 = 128) (i.e x = x/128)

            if x > 0 {
                encoded_bytes |= 0x80
            }
            buff.put_u8(encoded_bytes);
        }

        Ok(())
    }

    fn decode(buff: &Bytes) -> Result<u32, MQTTError> {
        let mut result = 0;
        let mut shift = 0;

        for (i, &byte) in buff.iter().enumerate() {
            // The least significant seven bits of each byte encode the data
            let value = (byte & 0x7F) as u32;
            result |= value << shift;
            shift += 7;

            // Continuation bit: The most significant bit is used to indicate whether there are still more bytes the representation
            if (byte & 0x80) == 0 {
                return Ok(result)
            }
            
            // 0, 1, 2, 3 (the maximum possible value that we expect is 268_435_455)
            if i >= 3 {
                return Err(MQTTError::MalformedPacket); 
            }
        }

        Err(MQTTError::IncompletePacket)
    }
}