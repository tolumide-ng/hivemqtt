// use super::property::Property::{self, *};

pub(crate) struct WillProperties {}


// impl WillProperties {
//     const PACKET_PROPERTY: u64 = packet_props!(PayloadFormatIndicator, MessageExpiryInterval, ContentType, ResponseTopic, CorrelationData, WillDelayInterval, UserProperty);

//     pub(crate) fn get_properties(&self) -> Vec<Property> {
//         let mut properties = Self::PACKET_PROPERTY;

//         let mut found = Vec::new();

//         while properties != 0 {
//             // equivalent to the enum's discriminant
//             let index = properties.trailing_zeros() as u8;
//             // We can do this because we know that the internal PACKET_PROPERTY above would always be valid since we generated it ourself
//             let property: Property = unsafe {std::mem::transmute(index)};
//             found.push(property);
//             properties &= properties-1;
//         }
//         found
//     }

//     pub(crate) fn has_property(&self, property: Property) -> bool {
//         ((1u64 << property as u64) & Self::PACKET_PROPERTY ) != 0
//     }
// }