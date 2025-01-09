use bytes::{Bytes, BytesMut};
use crate::v5::commons::fixed_header::FixedHeader;
use crate::v5::commons::property::Property;
use crate::v5::traits::{syncx::write::Write, syncx::read::Read};

use crate::v5::commons::error::MQTTError;

pub(crate) trait BufferIO: Sized {
    fn variable_length(&self) -> usize {
        if self.length() >= 2_097_152 { 4 }
        else if self.length() >= 16_384 { 3 }
        else if self.length() >= 128 { 2 }
        else { 1 }
    }

    /// Encodes a non-negative Integer into the Variable Byte Integer encoding
    fn encode(&self, buf: &mut BytesMut) -> Result<(), MQTTError> {
        let mut len = self.length();

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
    fn decode(buf: &mut Bytes) -> Result<(usize, usize), MQTTError> {
        let mut result = 0;

        for i in 0..4 {
            if buf.is_empty() {
                return Err(MQTTError::MalformedPacket);
            }
            let byte = u8::read(buf)?;

            result += ((byte as usize) & 0x7F) << (7 * i);

            if (byte & 0x80) == 0 {
                return Ok((result, i + 1))
            }
        }

        
        return Err(MQTTError::MalformedPacket)
    }
    
    /// Applies to fields that results in Protocol Error if their value appears more than once
    fn try_update<T>(field: &mut Option<T>, value: Option<T>) -> impl Fn(Property) -> Result<(), MQTTError> {
        let is_duplicate = field.is_some();
        *field = value;

        move |ppt| {
            if is_duplicate { return Err(MQTTError::DuplicateProperty(ppt.to_string())) }
            return Ok(())
        }
    }

    /// Allows a struct specify what it's length is to it's external users
    /// Normally this is obtainable using the .len() method (internally on structs implementing Length(formerly DataSize)),
    /// However, this method allows the struct customize what its actual length is.
    /// NOTE: The eventual plan is to make this the only property accessible externally and 
    ///     make `.len()` internal while probably enforcing that all struct's implementing this method/trait
    ///     must also implement `DataSize` proc. So that there is a default accurate length property
    fn length(&self) -> usize { 0 }

    fn read(_buf: &mut Bytes) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }

    fn read_with_fixedheader(_buf: &mut Bytes, _header: FixedHeader) -> Result<Self, MQTTError> {
        Err(MQTTError::MalformedPacket)
    }


    fn parse_len(buf: &mut Bytes) -> Result<Option<usize>, MQTTError> 
        where Self: Default {
        let (len, _) = Self::decode(buf)?;
        let self_str = "";

        if len == 0 { return Ok(None) }
        if len > buf.len() { return Err(MQTTError::IncompleteData(self_str, len, buf.len()))};

        Ok(Some(len))
    }

    fn w(&self, _buf: &mut BytesMut) {}

    fn write(&self, _buf: &mut BytesMut) -> Result<(), MQTTError> {
        Ok(())
    }
}