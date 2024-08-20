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


#[derive(Debug, Clone, Copy)]
pub struct ReasonCode;

impl ReasonCode {
    pub const SUCCESS: u64 = 0x00;
    pub const NORAMAL_DISCONNECTION: u64 = 0x00;
}
