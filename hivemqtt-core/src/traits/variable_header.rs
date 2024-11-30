use crate::commons::packet_type::PacketType;

/// Variable Header (Present in some MQTT Control Packets)
/// Last update: Chapter 2.1
pub(crate) trait VariableHeader {
    /// Optional: Two Byte Integer Packet Identifier field
    fn packet_identifier(packet: PacketType) {}
}