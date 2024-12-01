use crate::commons::packet_type::PacketType;
use crate::commons::property::Property::{self, *};

use super::variable_header::VariableHeader;

#[allow(unused_variables)]
type Properties = u64;
#[allow(unused_variables)]
const TOTAL_PACKETS: usize = 15;

// Open to further changes
trait Packet: VariableHeader {
    fn packet_type(&self) -> PacketType;
    fn encode(&self) -> [u8];
    fn decode(&self) -> Self
    where Self:Sized;


    /// The remaining bits [3-0] of byte 1 in the fixed header (Respective flag)
    fn flag(&self) -> u8 {
        let packet = self.packet_type();

        match packet {
            PacketType::Publish(p_flag) => p_flag,
            PacketType::PubRel | PacketType::Subscribe | PacketType::UnSubscribe => 0b0000_0010,
            _ => 0
        }
    }

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
    fn fixed_header(&self) -> u8 {
        let packet = self.packet_type();
        u8::from(packet) << Self::PACKET_TYPE_OFFSET | self.flag()
    }

    // /// Just like chess bitboards
    // const PACKET_PROPERTY: [Properties; TOTAL_PACKETS] = [
    //     packet_props!(SessionExpiryInterval, AuthenticationMethod, AuthenticationData, RequestProblemInformation, 
    //         RequestResponseInformation, ReceiveMaximum, TopicAliasMaximum, UserProperty, MaximumPacketSize), // CONNECT
    //     packet_props!(SessionExpiryInterval, AssignedClientIdentifier, ServerKeepAlive, AuthenticationMethod, AuthenticationData, 
    //         ResponseInformation, ServerReference, ReasonString, ReceiveMaximum, TopicAliasMaximum, MaximumQoS, RetainAvailable, UserProperty, 
    //         MaximumPacketSize, WildCardSubscription, SubscriptionIdentifierAvailable, SharedSubscriptionAvailable), // CONNACK
    //     packet_props!(PayloadFormatIndicator, MessageExpiryInterval, ContentType, ResponseTopic, CorrelationData, SubscriptionIdentifier, 
    //         TopicAliasMaximum, UserProperty), // Publish
    //     packet_props!(ReasonString, UserProperty), // PubAck
    //     packet_props!(ReasonString, UserProperty), // PubRecv
    //     packet_props!(ReasonString, UserProperty), // PubRel
    //     packet_props!(ReasonString, UserProperty), // PubComp
    //     packet_props!(SubscriptionIdentifier, UserProperty), // Subscribe
    //     packet_props!(ReasonString, UserProperty), // Suback
    //     packet_props!(UserProperty), // Unsubscribe 
    //     packet_props!(ReasonString, UserProperty), // Unsuback
    //     0u64, // Pingreq
    //     0u64, // Pingresp
    //     packet_props!(SessionExpiryInterval, ServerReference, ReasonString, UserProperty), // Disconnect
    //     packet_props!(AuthenticationMethod, AuthenticationData, ReasonString, UserProperty), // Auth
    // ];

    // fn get_properties(&self) -> Vec<Property> {
    //     let packet = self.packet_type();
    //     let index = (u8::from(packet)-1) as usize;
    //     let mut properties = Self::PACKET_PROPERTY[index];

    //     let mut found = Vec::new();

    //     while properties != 0 {
    //         // equivalent to the enum's discriminant
    //         let index = properties.trailing_zeros() as u8;
    //         // We can do this because we know that the internal PACKET_PROPERTY above would always be valid since we generated it ourself
    //         let property: Property = unsafe {std::mem::transmute(index)};
    //         found.push(property);
    //         properties &= properties-1;
    //     }
    //     found
    // }

    // fn has_property(&self, property: Property) -> bool {
    //     let packet = self.packet_type();
    //     let index = (u8::from(packet)-1) as usize;
    //     let properties = Self::PACKET_PROPERTY[index];
    //     ((1u64 << property as u64) & properties) != 0
    // }
}