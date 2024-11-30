use crate::commons::{fixed_header_flag::FixedHeaderFlag, packet_type::PacketType, qos::QoS};

/// Variable Header (Present in some MQTT Control Packets)
/// Last update: Chapter 2.1
pub(crate) trait VariableHeader {
    /// Optional: Two Byte Integer Packet Identifier field
    fn has_packet_identifier(packet: PacketType) -> bool {
        match packet {
            PacketType::Publish(value) => FixedHeaderFlag::from(value).qos > QoS::Zero,
            PacketType::PubAck | PacketType::PubRec | PacketType::PubRel | 
                PacketType::PubComp | PacketType::Subscribe | PacketType::SubAck | 
                PacketType::UnSubscribe | PacketType::UnSubAck => true,
            _ => false
        }
    }

    fn get(&self) -> String;
}