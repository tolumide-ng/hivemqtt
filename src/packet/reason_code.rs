// #[repr(u8)]
// pub(crate) enum ReasonCode {
//     /// CONNACK, PUBACK. PUBREC, PUBREL, PUBCOMP, UNSUBACK, AUTH (0x00)
//     Success,  // 0x00
//     /// NormalDisconnection = 0x00
//     NormalDisconnection,
//     /// SUBACK  = 0x00
//     GrantedQoS0,
//     /// SUBACK
//     GrantedQoS1 = 1,
//     /// SUBACK
//     GrantedQoS2 = 2,
// }


// #[derive(Debug, Clone, Copy)]
// pub struct ReasonCode(pub u8);

// impl ReasonCode {
//     pub const SUCCESS: ReasonCode = ReasonCode(0);
//     pub const NORAMAL_DISCONNECTION: ReasonCode = ReasonCode(0);
// }