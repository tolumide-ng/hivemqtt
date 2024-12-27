use std::io::{self, Error, ErrorKind};

use bytes::{Bytes, BytesMut};

use crate::v5::{commons::{error::MQTTError, fixed_header::FixedHeader, packet_type::PacketType}, traits::bufferio::BufferIO};

// pub trait AsyncStream: Send {}
// impl AsyncStream for ::smol::net::TcpStream {}
// impl AsyncStream for ::tokio::net::TcpStream {}

pub struct Stream {
    stream: ::smol::net::TcpStream,
    read_buffer: Bytes,
    write_buffer: BytesMut,
    const_buffer: [u8; 4096],
}

impl Stream {
    // fn  read(&mut self) -> io::Result<()> {
    //     let FixedHeader { packet_type, flags, remaining_length, header_len } = FixedHeader::read(&mut self.read_buffer).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    //     Ok(())
    // }

    // fn write(&self, packet: &Packet) -> Result<(), MQTTError> {
    //     // packet
    //     Ok(())
    // }
}