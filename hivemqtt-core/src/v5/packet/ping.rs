use crate::v5::{commons::{error::MQTTError, fixed_header::FixedHeader, packet_type::PacketType}, traits::syncx::bufferio::BufferIO};

#[derive(Debug, Default)]
pub struct PingReq {}

impl BufferIO for PingReq {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(PacketType::PingReq, 0, 0).write(buf)?;
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
        FixedHeader::new(PacketType::PingResp, 0, 0).write(buf)?;
        Ok(())
    }

    fn read(_buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
        Ok(Self::default())
    }
}