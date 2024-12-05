use bytes::{BufMut, BytesMut};

use crate::commons::error::MQTTError;

pub(crate) trait ControlPacket {
    fn w(&self, buf: &mut BytesMut);

    fn get_variable_length(len: usize) -> usize {
        if len >= 2_097_152 { 4 }
        else if len >= 16_384 { 3 }
        else if len >= 128 { 2 }
        else { 1 }
    }

    fn encode_variable_length(&self, buf: &mut BytesMut, len: usize) -> Result<usize, MQTTError> {
         // 268_435_455
        if len > 0xFFFFFFF {
            return Err(MQTTError::PayloadTooLong)
        }
        let mut x: usize = len;
        let mut count = 0;

        while x != 0 {
            // Get the last 7bits of x;
            let mut bytes = (x & 0x7F) as u8; // (x%128)
            x >>= 7; // shift to right by 7bits(i.e. 2^7 = 128) (i.e x = x/128)

            if x > 0 {
                bytes |= 0x80
            }
            buf.put_u8(bytes);
            count += 1;
        }

        Ok(count)
    }

    fn decode_variable_length(&self, buf: &mut BytesMut) {}

    /// Writes the length of the bytes and itself into the buffer
    fn ws(&self, buf: &mut BytesMut, value: &[u8]) {
        buf.put_u16(value.len() as u16);
        buf.extend_from_slice(value);
    }

    /// Allows a struct specify what it's length is to it's external users
    /// Normally this is obtainable using the .len() method (internally on structs implementing Length(formerly DataSize)),
    /// However, this method allows the struct customize what its actual length is.
    /// NOTE: The eventual plan is to make this the only property accessible externally and 
    ///     make `.len()` internal while probably enforcing that all struct's implementing this method/trait
    ///     must also implement `DataSize` proc. So that there is a default accurate length property
    fn length(&self) -> usize { 0 }
}