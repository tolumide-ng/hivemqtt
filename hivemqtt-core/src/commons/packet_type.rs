

// Position: byte 1, bits 7 - 4 (4 bits unsigned value)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    /// Client -> Server (Connection Request)
    Connect = 1,
    /// Server -> Client (Connection Acknowledgement)
    ConnAck,
    /// Client -> Sever | Server -> Client (Publish Message)
    Publish,
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

impl PacketType {
    #[allow(unused_variables)]
    const CONTROL_TYPE_MASK: u8 = 4;

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
    pub(crate) fn fixed_header(&self, flag: Option<u8>) -> u8 {
        (*self as u8) << Self::CONTROL_TYPE_MASK | self.flag(flag)
    }

    /// The remaining bits [3-0] of byte 1 in the fixed header (Respective flag)
    /// for Publish flag:
    ///     - bit 3 -> Duplicate delivery of PUBLISH packet
    ///     - bit 2 & bit 1 -> Publish Quality of Service(QoS)
    ///     - bit 0 -> Public retained message flag
    fn flag(&self, bits: Option<u8>) -> u8 {
        match &self {
            // find a better way to check if it supports dup, QoS type, and retain. We already know it's MQTT5
            Self::Publish => bits.unwrap(),
            Self::PubRel | Self::Subscribe | Self::UnSubscribe => 0b0000_0010,
            _ => 0
        }
    }

    /// Variable Byte Integer representing the number of bytes remaining within the current Control Packet
    /// (Size of Data in the Vairable Header + Size of Data in the Payload) in bytes
    /// 2.1.4
    pub(crate) fn remaining_length(&mut self, length: usize) {}
}