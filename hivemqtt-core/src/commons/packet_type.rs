

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
    /// Client -> Sever | Server -> Client (Publish received (QoS delivery part 1))
    PubRec,
    /// Client -> Sever | Server -> Client (Publish release (QoS delivery part 2))
    PubRel,
    /// Client -> Sever | Server -> Client (Publish complete (QoS 2 delivery part 3))
    PubComp,
    /// Client -> Server (Subscribe request)
    Subscribe,
    /// Server -> Client (Subcribe acknowledgement)
    SubAck,
    /// Client -> Server Unsubscribe request
    Unsubscribe,
    /// Server -> Client (Unsubscribe acknowledgement)
    UnsubAck,
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
    fn to_4bits(&self) -> u8 {
        (*self as u8) << 4
    }

    /// Fixed Header
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
    pub(crate) fn fixed_header(&self, flag: Option<u8>) -> u8 {
        self.to_4bits() | flag.unwrap_or(0u8)
    }
}