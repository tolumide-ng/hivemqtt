use super::qos::QoS;

/// This is currently only Applicable to `Public Packet` \
/// Client -> Sever | Server -> Client (Publish Message) \
/// The u8 here signifies: \
///     -> 0b0000_x000 - Duplicate delivery of a PUBLISH packet. \
///     -> 0b0000_0xx0 - Quality of Service (QoS) \
///     -> 0b0000_000x - Retained message flag \
pub(crate) struct FixedHeaderFlag {
    pub(crate) qos: QoS,
    /// Whether or not to enable duplicate delivery
    pub(crate) duplicate: bool,
    /// PUBLISH retained message flag
    pub(crate) retained: bool,
}


#[allow(unused_variables)]
const DUPLICATE_DELIVERY_MASK: u8 = 0b0000_1000;
#[allow(unused_variables)]
const QOS_MASK: u8 = 0b0000_0110;
const RETAIN_MASK: u8 = 0b0000_0001;


#[allow(unused_variables)]
const DUPLICATE_DELIVERY_OFFSET: u8 = 3;
#[allow(unused_variables)]
const QOS_OFFSET: u8 = 1;

impl FixedHeaderFlag {
        pub(crate) fn new(qos: QoS, duplicate: bool, retained: bool) -> Self {
        Self { qos, duplicate, retained }
    }
}


impl From<FixedHeaderFlag> for u8 {
    fn from(value: FixedHeaderFlag) -> Self {
        let FixedHeaderFlag { qos, duplicate, retained } = value;
        u8::from(duplicate) << DUPLICATE_DELIVERY_OFFSET | u8::from(qos) << QOS_OFFSET | u8::from(retained)
    }
}


impl From<u8> for FixedHeaderFlag {
    fn from(value: u8) -> Self {
        let duplicate = (value & DUPLICATE_DELIVERY_MASK).count_ones() > 0;
        let qos = QoS::from(value & QOS_MASK);
        let retained = (value * RETAIN_MASK).count_ones() > 0;

        Self { qos, duplicate, retained }
    }
}