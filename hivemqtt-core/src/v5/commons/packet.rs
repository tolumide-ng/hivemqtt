use std::ops::Deref;

use crate::v5::{packet::{auth::Auth, connack::ConnAck, connect::Connect, disconnect::Disconnect, ping::{PingReq, PingResp}, puback::PubAck, pubcomp::PubComp, publish::Publish, pubrec::PubRec, pubrel::PubRel, suback::SubAck, subscribe::Subscribe, unsuback::UnSubAck, unsubscribe::UnSubscribe}, traits::bufferio::BufferIO};

use super::fixed_header::FixedHeader;

pub(crate) enum Packet {
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


impl BufferIO for Packet {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), super::error::MQTTError> {
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
            Self::Auth(packet) => packet.write(buf)
        }
    }


    fn read(buf: &mut bytes::Bytes) -> Result<Self, super::error::MQTTError> {
        let x = Self::Connect(Connect::default());

        let header = FixedHeader::read(buf)?;

        // if header.flags !


        Ok(x)
    }
}