use std::borrow::Cow;

use bytes::BufMut;
use hivemqtt_macros::Length;

use crate::{commons::{packets::Packet, property::Property, qos::QoS}, traits::write::ControlPacket};

pub struct  Subscribe {
    packet_identifier: u16,
    properties: Option<SubcribeProperties>,
    payload: Vec<(String, SubscriptionOptions)>,
}


impl ControlPacket for Subscribe {
    /// (Length of Variable Header + Length of the Payload)
    fn length(&self) -> usize {
        let mut len = 0;

        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Subscribe as u8 | 1 << 1);
        //  Encoded as Variable Byte Integer
        let _ = Self::encode_variable_length(buf, self.length());

    }
}



#[derive(Debug, Length)]
pub struct SubcribeProperties {
    subscription_id: Option<usize>,
    user_property: Vec<(String, String)>,
}

impl ControlPacket for SubcribeProperties {
    fn length(&self) -> usize { self.len() }

    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = Self::encode_variable_length(buf, self.length());
        
        if let Some(id) = self.subscription_id {
            Property::SubscriptionIdentifier(Cow::Borrowed(&vec![id])).w(buf);
            Property::UserProperty(Cow::Borrowed(&self.user_property)).w(buf);
        }
    }
}


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


#[repr(u8)]
pub enum RetainHandling {
    /// Send the retained messages at the time of the subscribe
    Zero = 0,
    /// Send retained messages at subscribe only if subscription does not currently exist
    One = 1,
    /// Do not send reatined messages at the time of the subscription
    Two = 2,
}