use bytes::{Buf, Bytes};

use crate::commons::error::MQTTError;

pub trait Read: Sized {
    fn read(buf: &mut Bytes) -> Result<Self, MQTTError>;
}


impl Read for u8 {
    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let len = std::mem::size_of::<u8>();
        if buf.is_empty() { return Err(MQTTError::IncompleteData("u8", len, buf.len()))}
        
        Ok(buf.get_u8())
    }
}

impl Read for u16 {
    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let len = std::mem::size_of::<u16>();

        if buf.len() < len { return Err(MQTTError::IncompleteData("u16", len, buf.len()))}
        
        Ok(buf.get_u16())
    }
}

impl Read for u32 {
    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let len = std::mem::size_of::<u32>();
        if buf.len() < len { return Err(MQTTError::IncompleteData("u32", len, buf.len()))}
        
        Ok(buf.get_u32())
    }
}
