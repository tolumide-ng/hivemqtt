use hivemqtt_macros::FromU8;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromU8)]
pub(crate) enum Packet {
    Connect = 0x10, // 0b0001_0000
    ConnAck = 0x20, // 0b0010_0000
    Publish = 0x30, // 0b0011_0000
    PubAck = 0x40, // 0b0100_0000
    PubRec = 0x50, // 0b0101_0000
    PubRel = 0x60, // 0b0110_0000
    PubComp = 0x70, // 0b0111_0000
    Subscribe = 0x80, // 0b1000_0000
    SubAck = 0x90, // 0b1001_0000
    UnSubscribe = 0xA0, // 0b1010_0000
    UnSubAck = 0xB0, // 0b1011_0000
    PingReq = 0xC0, // 0b1100_0000
    PingResp = 0xD0, // 0b1101_0000
    Disconnect = 0xE0, // 0b1110_0000
    Auth = 0xF0, // 0b1111_0000
}


#[cfg(test)]
mod packet_type {
    use super::Packet;


    #[test]
    fn should_return_the_right_enum_discriminant() {
        assert_eq!(u8::from(Packet::PubAck), 64);
        assert_eq!(u8::from(Packet::Connect), 16);
        assert_eq!(u8::from(Packet::Publish), 48);
        assert_eq!(u8::from(Packet::Auth), 240);
    }
}


