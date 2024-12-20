use crate::{commons::{fixed_header::FixedHeader, packets::Packet}, traits::bufferio::BufferIO};

#[derive(Debug, Default)]
pub struct PingReq {}

impl BufferIO for PingReq {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        FixedHeader::new(Packet::PingReq, 0, 0).write(buf)?;
        Ok(())
    }

    fn read(_buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        Ok(Self::default())
    }
}


#[derive(Debug, Default)]
pub struct PingResp;

impl BufferIO for PingResp {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), crate::commons::error::MQTTError> {
        FixedHeader::new(Packet::PingResp, 0, 0).write(buf)?;
        Ok(())
    }

    fn read(_buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        Ok(Self::default())
    }
}