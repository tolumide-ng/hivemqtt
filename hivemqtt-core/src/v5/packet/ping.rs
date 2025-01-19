use crate::v5::{
    commons::{error::MQTTError, fixed_header::FixedHeader, packet_type::PacketType},
    traits::syncx::bufferio::BufferIO,
};

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

mod syncx {
    use crate::v5::{commons::error::MQTTError, traits::bufferio::BufferIO};

    use super::{FixedHeader, PacketType, PingReq, PingResp};

    impl BufferIO for PingReq {
        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            FixedHeader::new(PacketType::PingReq, 0, 0).write(buf)?;
            Ok(())
        }

        fn read(_buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
            Ok(Self::default())
        }
    }

    impl BufferIO for PingResp {
        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            FixedHeader::new(PacketType::PingResp, 0, 0).write(buf)?;
            Ok(())
        }

        fn read(_buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
            Ok(Self::default())
        }
    }
}

mod asyncx {
    use crate::v5::traits::streamio::StreamIO;

    use super::{FixedHeader, PacketType, PingReq, PingResp};

    impl StreamIO for PingReq {
        async fn write<W>(&self, stream: &mut W) -> Result<(), crate::v5::commons::error::MQTTError>
        where
            W: futures::AsyncWriteExt + Unpin,
        {
            FixedHeader::new(PacketType::PingReq, 0, 0)
                .write(stream)
                .await
        }

        async fn read<R>(stream: &mut R) -> Result<Self, crate::v5::commons::error::MQTTError>
        where
            R: futures::AsyncReadExt + Unpin,
            Self: Default,
        {
            Ok(Self::default())
        }
    }

    impl StreamIO for PingResp {
        async fn write<W>(&self, stream: &mut W) -> Result<(), crate::v5::commons::error::MQTTError>
        where
            W: futures::AsyncWriteExt + Unpin,
        {
            FixedHeader::new(PacketType::PingResp, 0, 0)
                .write(stream)
                .await
        }

        async fn read<R>(stream: &mut R) -> Result<Self, crate::v5::commons::error::MQTTError>
        where
            R: futures::AsyncReadExt + Unpin,
            Self: Default,
        {
            Ok(Self::default())
        }
    }
}
