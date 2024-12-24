use crate::v5::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet}, traits::bufferio::BufferIO};

#[derive(Debug, Default)]
pub struct PingReq {}

impl BufferIO for PingReq {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::PingReq, 0, 0).write(buf)?;
        Ok(())
    }

    fn read(_buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        Ok(Self::default())
    }
}


#[derive(Debug, Default)]
pub struct PingResp;

impl BufferIO for PingResp {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::PingResp, 0, 0).write(buf)?;
        Ok(())
    }

    fn read(_buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        Ok(Self::default())
    }
}