use bytes::{Buf, BufMut};

use crate::commons::error::MQTTError;
use crate::commons::qos::QoS;

use crate::traits::{read::Read, write::Write, bufferio::BufferIO};




#[derive(Debug, Clone, Copy)]
pub struct SubscriptionOptions {
    qos: QoS,
    no_local: bool,
    retain_as_published: bool,
    retain_handling: RetainHandling
}

impl From<SubscriptionOptions> for u8 {
    fn from(v: SubscriptionOptions) -> Self {
        u8::from(v.qos) | u8::from(v.no_local) << 2 | u8::from(v.retain_as_published) << 3 | (v.retain_handling as u8) << 4
    }
}

impl BufferIO for SubscriptionOptions {
    fn length(&self) -> usize { 1 }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(u8::from(*self));
    }

    fn read(buf: &mut bytes::Bytes) -> Result<Self, crate::commons::error::MQTTError> {
        let byte = buf.get_u8();

        let qos = QoS::try_from(byte & 0b0000_0011)?;
        let no_local = (byte & 0b0000_0100) != 0;
        let retain_as_published = (byte & 0b0000_1000) != 0;
        let retain_handling = RetainHandling::try_from(byte & 0b0011_0000)?;

        Ok(Self { qos, no_local, retain_as_published, retain_handling })
    }
}



#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RetainHandling {
    /// Send the retained messages at the time of the subscribe
    Zero = 0,
    /// Send retained messages at subscribe only if subscription does not currently exist
    One = 1,
    /// Do not send retained messages at the time of the subscription
    Two = 2,
}

impl TryFrom<u8> for RetainHandling {
    type Error = MQTTError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(MQTTError::MalformedPacket)
        }
    }
    
}
