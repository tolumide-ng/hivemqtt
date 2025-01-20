use bytes::Bytes;

use crate::v5::{
    packet::{
        auth::Auth,
        connack::ConnAck,
        connect::Connect,
        disconnect::Disconnect,
        ping::{PingReq, PingResp},
        puback::PubAck,
        pubcomp::PubComp,
        publish::Publish,
        pubrec::PubRec,
        pubrel::PubRel,
        suback::SubAck,
        subscribe::Subscribe,
        unsuback::UnSubAck,
        unsubscribe::UnSubscribe,
    },
    traits::read_data::ReadData,
};

use super::{error::MQTTError, fixed_header::FixedHeader, packet_type::PacketType};

#[derive(Debug)]
pub enum Packet {
    Connect(Connect),
    ConnAck(ConnAck),
    Publish(Publish),
    PubAck(PubAck),
    PubRec(PubRec),
    PubRel(PubRel),
    PubComp(PubComp),
    Subscribe(Subscribe),
    SubAck(SubAck),
    UnSubscribe(UnSubscribe),
    UnSubAck(UnSubAck),
    PingReq(PingReq),
    PingResp(PingResp),
    Disconnect(Disconnect),
    Auth(Auth),
}

impl Default for Packet {
    fn default() -> Self {
        unimplemented!()
    }
}

impl ReadData for Packet {}

pub(crate) mod syncx {
    use super::*;
    // use super::{Connect, FixedHeader, Packet, PacketType};
    use crate::v5::{commons::error::MQTTError, traits::bufferio::BufferIO};

    impl BufferIO for Packet {
        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            match self {
                Self::Connect(packet) => packet.write(buf),
                Self::ConnAck(packet) => packet.write(buf),
                Self::Publish(packet) => packet.write(buf),
                Self::PubAck(packet) => packet.write(buf),
                Self::PubRec(packet) => packet.write(buf),
                Self::PubRel(packet) => packet.write(buf),
                Self::PubComp(packet) => packet.write(buf),
                Self::Subscribe(packet) => packet.write(buf),
                Self::SubAck(packet) => packet.write(buf),
                Self::UnSubscribe(packet) => packet.write(buf),
                Self::UnSubAck(packet) => packet.write(buf),
                Self::PingReq(packet) => packet.write(buf),
                Self::PingResp(packet) => packet.write(buf),
                Self::Disconnect(packet) => packet.write(buf),
                Self::Auth(packet) => packet.write(buf),
            }
        }

        fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
            // let x = Self::Connect(Connect::default());

            let header = FixedHeader::read(buf)?;
            match header.packet_type {
                PacketType::Connect => Ok(Packet::Connect(Connect::read(buf)?)),
                PacketType::ConnAck => Ok(Packet::ConnAck(ConnAck::read(buf)?)),
                PacketType::Publish => Ok(Packet::Publish(Publish::read_with_fixedheader(
                    buf, header,
                )?)),
                PacketType::PubAck => {
                    Ok(Packet::PubAck(PubAck::read_with_fixedheader(buf, header)?))
                }
                PacketType::PubRec => {
                    Ok(Packet::PubRec(PubRec::read_with_fixedheader(buf, header)?))
                }
                PacketType::PubRel => {
                    Ok(Packet::PubRel(PubRel::read_with_fixedheader(buf, header)?))
                }
                PacketType::PubComp => Ok(Packet::PubComp(PubComp::read_with_fixedheader(
                    buf, header,
                )?)),
                PacketType::Subscribe => Ok(Packet::Subscribe(Subscribe::read(buf)?)),
                PacketType::SubAck => Ok(Packet::SubAck(SubAck::read(buf)?)),
                PacketType::UnSubscribe => Ok(Packet::UnSubscribe(UnSubscribe::read(buf)?)),
                PacketType::UnSubAck => Ok(Packet::UnSubAck(UnSubAck::read(buf)?)),
                PacketType::PingReq => Ok(Packet::PingReq(PingReq::read(buf)?)),
                PacketType::PingResp => Ok(Packet::PingResp(PingResp::read(buf)?)),
                PacketType::Auth => Ok(Packet::Auth(Auth::read(buf)?)),
                PacketType::Disconnect => Ok(Packet::Disconnect(Disconnect::read(buf)?)),
                _ => Err(MQTTError::UnknownData(format!(
                    "Unexpected Packet type {:?}",
                    header.packet_type
                ))),
            }
        }
    }
}

mod asyncx {
    use crate::v5::traits::streamio::StreamIO;

    use super::*;

    impl StreamIO for Packet {
        async fn write<W>(&self, stream: &mut W) -> Result<(), MQTTError>
        where
            W: futures::AsyncWriteExt + Unpin,
        {
            match self {
                Self::Connect(packet) => packet.write(stream).await,
                Self::ConnAck(packet) => packet.write(stream).await,
                Self::Publish(packet) => packet.write(stream).await,
                Self::PubAck(packet) => packet.write(stream).await,
                Self::PubRec(packet) => packet.write(stream).await,
                Self::PubRel(packet) => packet.write(stream).await,
                Self::PubComp(packet) => packet.write(stream).await,
                Self::Subscribe(packet) => packet.write(stream).await,
                Self::SubAck(packet) => packet.write(stream).await,
                Self::UnSubscribe(packet) => packet.write(stream).await,
                Self::UnSubAck(packet) => packet.write(stream).await,
                Self::PingReq(packet) => packet.write(stream).await,
                Self::PingResp(packet) => packet.write(stream).await,
                Self::Disconnect(packet) => packet.write(stream).await,
                Self::Auth(packet) => packet.write(stream).await,
                _ => Ok(()),
            }
        }

        async fn read<R>(stream: &mut R) -> Result<Self, MQTTError>
        where
            R: futures::AsyncReadExt + Unpin,
            Self: Default,
        {
            let header = FixedHeader::read(stream).await?;
            match header.packet_type {
                PacketType::Connect => Ok(Packet::Connect(Connect::read(stream).await?)),
                PacketType::ConnAck => Ok(Packet::ConnAck(ConnAck::read(stream).await?)),
                PacketType::Publish => Ok(Packet::Publish(
                    Publish::read_with_fixedheader(stream, &header).await?,
                )),
                PacketType::PubAck => Ok(Packet::PubAck(
                    PubAck::read_with_fixedheader(stream, &header).await?,
                )),
                PacketType::PubRec => Ok(Packet::PubRec(
                    PubRec::read_with_fixedheader(stream, &header).await?,
                )),
                PacketType::PubRel => Ok(Packet::PubRel(
                    PubRel::read_with_fixedheader(stream, &header).await?,
                )),
                PacketType::PubComp => Ok(Packet::PubComp(
                    PubComp::read_with_fixedheader(stream, &header).await?,
                )),
                PacketType::Subscribe => Ok(Packet::Subscribe(Subscribe::read(stream).await?)),
                PacketType::SubAck => Ok(Packet::SubAck(SubAck::read(stream).await?)),
                PacketType::UnSubscribe => {
                    Ok(Packet::UnSubscribe(UnSubscribe::read(stream).await?))
                }
                PacketType::UnSubAck => Ok(Packet::UnSubAck(UnSubAck::read(stream).await?)),
                PacketType::PingReq => Ok(Packet::PingReq(PingReq::read(stream).await?)),
                PacketType::PingResp => Ok(Packet::PingResp(PingResp::read(stream).await?)),
                PacketType::Auth => Ok(Packet::Auth(Auth::read(stream).await?)),
                PacketType::Disconnect => Ok(Packet::Disconnect(Disconnect::read(stream).await?)),
                _ => Err(MQTTError::UnknownData(format!(
                    "Unexpected Packet type {:?}",
                    header.packet_type
                ))),
            }
        }
    }
}

// impl Packet {
//     pub(crate) const CONNECT: u8 = 0x10; // 0b0001_0000
//     pub(crate) const CONNACK: u8 = 0x20; // 0b0010_0000
//     pub(crate) const PUBLISH: u8 = 0x30; // 0b0011_0000
//     pub(crate) const PUBACK: u8 = 0x40; // 0b0100_0000
//     pub(crate) const PUBREC: u8 = 0x50; // 0b0101_0000
//     pub(crate) const PUBREL: u8 = 0x60; // 0b0110_0000
//     pub(crate) const PUBCOMP: u8 = 0x70; // 0b0111_0000
//     pub(crate) const SUBSCRIBE: u8 = 0x80; // 0b1000_0000
//     pub(crate) const SUBACK: u8 = 0x90; // 0b1001_0000
//     pub(crate) const UNSUBSCRIBE: u8 = 0xA0; // 0b1010_0000
//     pub(crate) const UNSUBACK: u8 = 0xB0; // 0b1011_0000
//     pub(crate) const PINGREQ: u8 = 0xC0; // 0b1100_0000
//     pub(crate) const PINGRESP: u8 = 0xD0; // 0b1101_0000
//     pub(crate) const DISCONNECT: u8 = 0xE0; // 0b1110_0000
//     pub(crate) const AUTH: u8 = 0xF0; // 0b1111_0000
// }
