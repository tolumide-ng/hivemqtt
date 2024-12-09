use bytes::BufMut;

use crate::{commons::packets::Packet, traits::write::BufferIO};

pub struct PingReq {}

impl BufferIO for PingReq {
    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::PingReq as u8);
        buf.put_u8(0);
    }
}


pub struct PingRes {}

impl BufferIO for PingRes {
    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::PingResp as u8);
        buf.put_u8(0);
    }
}