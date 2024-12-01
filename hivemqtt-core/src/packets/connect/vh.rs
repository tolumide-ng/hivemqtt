use bytes::BytesMut;

use crate::commons::version::Version;

pub(crate) struct VariableHeader {
    protocol_level: Version, // 
    connect_flags: u8,
    keep_alive: bool,
    properties: Option<String>
}


impl VariableHeader {
    pub(crate) fn write(&self) {
        let mut buff = BytesMut::new();

        // buff.p
    }
}


// The will message consists of:
//  - Will Properties
//  - Will Topic
//  - Will Payload

// If the Will Flag is set to 1, the Will Properties, Will Topic and Will Payload fields MUST be present in the Payload.
// The Will message 