use super::packets::Packet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedHeader {
    pub packet_type: Packet,
    pub flags: u8,
    pub remaining_length: u8,
}

impl FixedHeader {
    fn write(packet: Packet) {}

    fn read() {}
}