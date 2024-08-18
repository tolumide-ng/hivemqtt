use super::packet_type::PacketType;



/// Fixed Header
/// ```text
///     +--------+------+-------+-------+-------+-------+-------+-------+
///     | Bit    |  7   |   6   |   5   |   4   |   3   |   2   |   1   |
///     +--------+------+-------+-------+-------+-------+-------+-------+
///     | byte 1 |  MQTT Control Packet type    | Respective flag       |
///     +--------+------+-------+-------+-------+-------+-------+-------+
///     | byte 2 |                  Remaining Length                    |
///     +--------+------+-------+-------+-------+-------+-------+-------+
/// ```
/// Each MQTT Control Packet contains a Fixed Header
#[derive(Debug, Clone, Copy)]
pub struct FixedHeader {
    /// byte 1, bits 7 - 4
    packet: PacketType,
    flags: u8,
    remaining_length: usize,
}

impl FixedHeader {
    pub(crate)  fn new(packet: PacketType, flags: u8, remaining_length: usize) -> Self {
        Self { packet, flags, remaining_length }
    }
}