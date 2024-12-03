use super::{fixed_header_flag::FixedHeaderFlag, property::Property::{self, *}};




// For an MQTT control packet, we expect
// 0. Fixed Header (Compulsory)
// 1. Variable Header (Compulsory with some parts of it optional)
// 2. Payload (Optional for some MQTT packets)




// Position: byte 1, bits 7 - 4 (4 bits unsigned value)
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum PacketType {
    Connect = 1,
    ConnAck = 2,
    Publish(u8) = 3, 
    PubAck,
    PubRec,
    PubRel,
    PubComp,
    Subscribe,
    SubAck,
    UnSubscribe,
    UnSubAck,
    PingReq,
    PingResp,
    Disconnect,
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

type Properties = u64;

impl PacketType {
    const TOTAL_PACKETS: usize = 15;
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


