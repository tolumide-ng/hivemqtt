use super::{fixed_header_flag::FixedHeaderFlag, property::Property::{self, *}};




// For an MQTT control packet, we expect
// 0. Fixed Header (Compulsory)
// 1. Variable Header (Compulsory with some parts of it optional)
// 2. Payload (Optional for some MQTT packets)




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

type Properties = u64;

impl PacketType {
    #[allow(unused_variables)]
    const PACKET_TYPE_OFFSET: u8 = 4;
    const TOTAL_PACKETS: usize = 15;

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
            Self::Publish(p_flag) => *p_flag,
            Self::PubRel | Self::Subscribe | Self::UnSubscribe => 0b0000_0010,
            _ => 0
        }
    }

    /// Integer representing the number of bytes in the Variable Header and the Payload.
    /// This is encoded is VBI(Variable Byte Integer)
    /// (Size of Data in the Variable Header + Size of Data in the Payload) in bytes
    /// 2.1.4
    pub(crate) fn remaining_length(&mut self, length: usize) {}


    pub(crate) fn make_publish(flag: FixedHeaderFlag) -> PacketType {
        Self::Publish(u8::from(flag))
    }

    /// Just like chess bitboards
    const PACKET_PROPERTY: [Properties; Self::TOTAL_PACKETS] = [
        packet_props!(SessionExpiryInterval, AuthenticationMethod, AuthenticationData, RequestProblemInformation, 
            RequestResponseInformation, ReceiveMaximum, TopicAliasMaximum, UserProperty, MaximumPacketSize), // CONNECT
        packet_props!(SessionExpiryInterval, AssignedClientIdentifier, ServerKeepAlive, AuthenticationMethod, AuthenticationData, 
            ResponseInformation, ServerReference, ReasonString, ReceiveMaximum, TopicAliasMaximum, MaximumQoS, RetainAvailable, UserProperty, 
            MaximumPacketSize, WildCardSubscription, SubscriptionIdentifierAvailable, SharedSubscriptionAvailable), // CONNACK
        packet_props!(PayloadFormatIndicator, MessageExpiryInterval, ContentType, ResponseTopic, CorrelationData, SubscriptionIdentifier, 
            TopicAliasMaximum, UserProperty), // Publish
        packet_props!(ReasonString, UserProperty), // PubAck
        packet_props!(ReasonString, UserProperty), // PubRecv
        packet_props!(ReasonString, UserProperty), // PubRel
        packet_props!(ReasonString, UserProperty), // PubComp
        packet_props!(SubscriptionIdentifier, UserProperty), // Subscribe
        packet_props!(ReasonString, UserProperty), // Suback
        packet_props!(UserProperty), // Unsubscribe 
        packet_props!(ReasonString, UserProperty), // Unsuback
        0u64, // Pingreq
        0u64, // Pingresp
        packet_props!(SessionExpiryInterval, ServerReference, ReasonString, UserProperty), // Disconnect
        packet_props!(AuthenticationMethod, AuthenticationData, ReasonString, UserProperty), // Auth
    ];

    pub(crate) fn get_properties(&self) -> Vec<Property> {
        let index = (u8::from(*self)-1) as usize;
        let mut properties = Self::PACKET_PROPERTY[index];

        let mut found = Vec::new();

        while properties != 0 {
            // equivalent to the enum's discriminant
            let index = properties.trailing_zeros() as u8;
            // We can do this because we know that the internal PACKET_PROPERTY above would always be valid since we generated it ourself
            let property: Property = unsafe {std::mem::transmute(index)};
            found.push(property);
            properties &= properties-1;
        }
        found
    }

    pub(crate) fn has_property(&self, property: Property) -> bool {
        let index = (u8::from(*self)-1) as usize;
        let properties = Self::PACKET_PROPERTY[index];
        ((1u64 << property as u64) & properties) != 0
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


