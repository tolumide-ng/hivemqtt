use super::connect_flags::ConnectFlags;

/// ```text
/// +--------+--------------------------+---+---+---+---+---+---+---+---+
/// |               |    Description    | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Protocol Name                                                 |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 1        | Length MSB(0)     |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 2        | Length LSB(4)     |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+ 
/// | byte 3        |       'M'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 4        |       'Q'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 5        |       'T'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 6        |       'T'         |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Protocol Version                                              |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 7        | Version(5)        |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// |               |   Description     | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// |               | User Name Flag(1) |   |   |   |   |   |   |   |   |
/// |               | Password Flag(1)  |   |   |   |   |   |   |   |   |
/// |               | Will Retain(0)    |   |   |   |   |   |   |   |   |
/// | byte 8        | Will QoS(01)      | 1 | 1 | 0 | 0 | 1 | 1 | 1 | 0 |
/// |               | Will Flag(1)      |   |   |   |   |   |   |   |   |
/// |               | Clean Start(1)    |   |   |   |   |   |   |   |   |
/// |               | Reserved(0)       |   |   |   |   |   |   |   |   |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Keep Alive                                            |   |       |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 9        | Keep Alive MSB(0) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 10       | Keep Alive LSB(10)| 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | Properties                                            |   |       |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 11       | Length(5)         | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 1 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 12       | SEII(17)          | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 1 |
/// +---------------+-------------------+---+---+---+---+---+---+---+---+
/// | byte 13       | SEI (10)          | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+                   +---+---+---+---+---+---+---+---+
/// | byte 14       |                   | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+                   +---+---+---+---+---+---+---+---+
/// | byte 15       |                   | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
/// +---------------+                   +---+---+---+---+---+---+---+---+
/// | byte 16       |                   | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 |
/// +-----------------------------------+---+---+---+---+---+---+---+---+
/// 
/// **SEII = Session Expiry Interval Identifier
/// **SEI  = Session Expiry Interval
/// ```
pub struct VariableHeader {
    connect_flags: ConnectFlags,
    // protocol_version: ProtocolVersion,
    /// The maximum value is 65_535(u16::MAX) = 18hours and 15seconds
    keep_alive: u16,
    /// The length of the Properties in the CONNECT packet Variable Header encoded as a Variable Byte Integer
    property_length: u16,
    /// 4 byte integer representing the Session Expiry inteval in seconds
    /// If this value is setabsent or set to 0, the Session ends when the Network is closed.
    /// If the value is set to 0xFFFFFFFF (u32::MAX), the Session does not expire
    session_expiry_interval: u32,
    /// This value must be greater than zero(0), else there would be a protocol error.
    /// The Client uses this value to limit the number of QoS1, and QoS2 publications that it is willing to concurrently process.
    /// If this value is absent, then its value defaults to u16::MAX
    receive_maximum: u16,
    /// This value indicates the maximum packet size the Client is willing to accept
    /// If this value is not present, there is no limit on the packet size.
    /// ! This value must be greater than zero(0) if/when set, else there would be a Protoxol error
    /// If the server sends a packet whose size exceeds the limit, It is a protocol error, and the client must disconnect
    /// with ReasonCode 0x95 (Packet too Large)
    maximum_packet_size: Option<u16>,
    /// Default is 0 if the value is absent (two bytes)
    /// The Client uses this value to limit the number of Topic Aliases that it is willing to hold on this Connection.
    topic_alias_maximum: u16,
    /// The only valid values here are 0 or 1, anything else would result in a protocl error
    /// The Client uses this value to request the Server to return Response information in the CONNACK.
    request_response_information: bool,
    /// Default value here is 1
    /// The only valid values here are 0 or 1, anything else would result in a Protocol Error
    /// The Client uses this to indicate whether the Reason String or User properties are snet in the case of failures
    request_response_problem: bool,
    /// UTF-8 String Pair
    user_property: String,
    // authentication: Option<ConnectAuthentication>,
    /// Name of the Authentication method used for extended authentication. 
    /// If the Client sets this value, the Client MUST NOT send any packets other than AUTH or 
    /// DISCONNECT packets until it has received a CONNACK packet
    authentication_method: String,
    /// Binary Data containing the authentication data.
    /// It is ProtocolError to include Authentication Data if there is no Authentication Method.
    authentication_data: String,
}

// struct ConnectAuthentication {
//     /// Name of the Authentication method used for extended authentication. 
//     /// If the Client sets this value, the Client MUST NOT send any packets other than AUTH or 
//     /// DISCONNECT packets until it has received a CONNACK packet
//     authentication_method: String,
//     /// Binary Data containing the authentication data.
//     /// It is ProtocolError to include Authentication Data if there is no Authentication Method.
//     authentication_data: String,
// }