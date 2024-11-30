use super::fixed_header::FixedHeaderFlag;



// Position: byte 1, bits 7 - 4 (4 bits unsigned value)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    /// Client -> Server (Connection Request)
    Connect = 1,
    /// Server -> Client (Connection Acknowledgement)
    ConnAck = 2,
    /// Client -> Sever | Server -> Client (Publish Message)
    /// The u8 here signifies:
    ///     -> 0b0000_000x - Duplicate delivery of a PUBLISH packet
    ///     -> 0b0000_0xx0 - Quality of Service (QoS)
    ///     -> 0b0000_x000 - Retained message flag
    Publish(u8) = 3, 
    /// Client -> Sever | Server -> Client (Publish acknowledgement (QoS 1))
    PubAck,
    /// Client -> Sever | Server -> Client (Publish received (QoS 2 delivery part 1))
    PubRec,
    /// Client -> Sever | Server -> Client (Publish release (QoS 2 delivery part 2))
    PubRel,
    /// Client -> Sever | Server -> Client (Publish complete (QoS 2 delivery part 3))
    PubComp,
    /// Client -> Server (Subscribe request)
    Subscribe,
    /// Server -> Client (Subcribe acknowledgement)
    SubAck,
    /// Client -> Server Unsubscribe request
    UnSubscribe,
    /// Server -> Client (Unsubscribe acknowledgement)
    UnSubAck,
    /// Client -> Server (PING request)
    PingReq,
    /// Server -> Client (PING response)
    PingResp,
    /// Client -> Sever | Server -> Client (Disconnect notification)
    Disconnect,
    /// Client -> Sever | Server -> Client (Authentication Exchange)
    Auth,
}

impl From<PacketType> for u8 {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::Publish(_) => 3,
            _ => unsafe { *(<*const _>::from(&value)).cast::<u8>() }
        }
    }
}

impl PacketType {
    #[allow(unused_variables)]
    const PACKET_TYPE_OFFSET: u8 = 4;

    /// Fixed Header (Present in all MQTT Control Packets)
    /// ```text
    /// +--------+------+-------+-------+-------+-------+-------+-------+-------+
    /// | Bit    |  7   |   6   |   5   |   4   |   3   |   2   |   1   |   0   |
    /// +--------+------+-------+-------+-------+-------+-------+-------+-------+
    /// | byte 1 |  MQTT Control Packet type    | Respective flag               |
    /// +--------+------+-------+-------+-------+-------+-------+-------+-------+
    /// | byte 2 |                  Remaining Length                    |       |
    /// +--------+------+-------+-------+-------+-------+-------+-------+-------+
    /// ```
    /// Each MQTT Control Packet contains a Fixed Header
    /// `flag` parameter only needs to be provided when the packet type is `Publish`
    ///     - (bool, bool) -> (duplicate delivery, publish retained message flag)
    pub(crate) fn fixed_header(&self) -> u8 {
        u8::from(*self) << Self::PACKET_TYPE_OFFSET | self.flag()
    }

    /// The remaining bits [3-0] of byte 1 in the fixed header (Respective flag)
    fn flag(&self) -> u8 {
        match &self {
            // find a better way to check if it supports dup, QoS type, and retain. We already know it's MQTT5
            Self::Publish(p_flag) => *p_flag,
            Self::PubRel | Self::Subscribe | Self::UnSubscribe => 0b0000_0010,
            _ => 0
        }
    }

    /// Variable Byte Integer representing the number of bytes remaining within the current Control Packet
    /// (Size of Data in the Vairable Header + Size of Data in the Payload) in bytes
    /// 2.1.4
    pub(crate) fn remaining_length(&mut self, length: usize) {}


    pub(crate) fn make_publish(flag: FixedHeaderFlag) -> PacketType {
        Self::Publish(u8::from(flag))
    }
}



#[cfg(test)]
mod packet_type {
    use super::PacketType;

    #[test]
    fn should_return_the_right_enum_discriminant() {
        assert_eq!(u8::from(PacketType::PubAck), 4);
        assert_eq!(u8::from(PacketType::Connect), 1);
        assert_eq!(u8::from(PacketType::Publish(4)), 3);
        assert_eq!(u8::from(PacketType::Auth), 15);
    }
}