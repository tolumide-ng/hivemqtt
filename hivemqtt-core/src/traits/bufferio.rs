use bytes::{BufMut, Bytes, BytesMut};

use crate::commons::error::MQTTError;

pub(crate) trait BufferIO: Sized {

    fn w(&self, buf: &mut BytesMut);

    fn get_variable_length(len: usize) -> usize {
        if len >= 2_097_152 { 4 }
        else if len >= 16_384 { 3 }
        else if len >= 128 { 2 }
        else { 1 }
    }

    fn variable_length(&self) -> usize {
        if self.length() >= 2_097_152 { 4 }
        else if self.length() >= 16_384 { 3 }
        else if self.length() >= 128 { 2 }
        else { 1 }
    }

    fn write_variable_integer(buf: &mut BytesMut, len: usize) -> Result<usize, MQTTError> {
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

    fn read_variable_integer(buf: &mut Bytes) -> Result<(usize, usize), MQTTError> {
        let mut result = 0;
        let mut shift = 0;

        for (len, &byte) in buf.iter().enumerate() {
            // The least significant seven bits of each byte encode the data
            result |= (byte as usize & 0x7F) << shift;
            shift += 7;

            // Continuation bit: The most significant bit is used to indicate whether there are still more bytes the representation
            if (byte & 0x80) == 0 {
                return Ok((result, len))
            }
            
            // 0, 1, 2, 3 (the maximum possible value that we expect is 268_435_455)
            if len >= 3 {
                break;
            }
        }

        return Err(MQTTError::MalformedPacket); 
    }

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

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }
}