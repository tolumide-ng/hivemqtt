use bytes::Bytes;

/// Must be encoded using the VBI
#[derive(Debug, Clone)]
#[repr(u8)]
pub(crate) enum Property {
    PayloadFormatIndicator(Option<u8>) = 1,
    MessageExpiryInterval(Option<u32>) = 2,
    ContentType(Option<String>) = 3,
    ResponseTopic(Option<String>) = 8,
    CorrelationData(Option<Bytes>) = 9,
    // this can be a Option<usize> or Vec<usize>, we can create an extra enum for this if there is a need for it.
    SubscriptionIdentifier(Option<usize>) = 11, 
    SessionExpiryInterval(Option<u32>) = 17,
    AssignedClientIdentifier(Option<String>) = 18,
    ServerKeepAlive(Option<u16>) = 19,
    AuthenticationMethod(Option<String>) = 21,
    AuthenticationData(Option<Bytes>) = 22,
    RequestProblemInformation(Option<u8>) = 23,
    WillDelayInterval(Option<u32>) = 24,
    RequestResponseInformation(Option<u8>) = 25,
    ResponseInformation(Option<String>) = 26,
    ServerReference(Option<String>) = 28,
    ReasonString(Option<String>) = 31,
    ReceiveMaximum(Option<u16>) = 33,
    TopicAliasMaximum(Option<u16>) = 34,
    MaximumQoS(Option<u8>) = 36,
    RetainAvailable(Option<u8>) = 37,
    UserProperty(Vec<(String, String)>) = 38,
    MaximumPacketSize(Option<u32>) = 39,
    WildCardSubscription(Option<u8>) = 40,
    SubscriptionIdentifierAvailable(Option<u8>) = 41,
    SharedSubscriptionAvailable(Option<u8>) = 42,
}

// impl From<Property> for u8 {
//     fn from(value: Property) -> Self {
//         let result = value as u8;
//         return result
//     }
// }


