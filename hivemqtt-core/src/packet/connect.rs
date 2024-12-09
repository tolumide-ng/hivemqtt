use std::borrow::Cow;

use bytes::{BufMut, Bytes};
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, packets::Packet, property::Property, qos::QoS, variable_byte_integer::{variable_integer, variable_length}, version::Version}, constants::PROTOCOL_NAME, traits::bufferio::BufferIO};

#[derive(Debug, Default)]
pub(crate) struct ConnectFlags {
    pub(super) username: bool,
    pub(super) password: bool,
    pub(super) will_retain: bool,
    pub(super) will_qos: QoS,
    pub(super) will_flag: bool,
    pub(super) clean_start: bool,
}

impl ConnectFlags {
    const USERNAME_MASK: u8 = 1 << 7;
    const PASSWORD_MASK: u8 = 1 << 6;
    const WILL_RETAIN_MASK: u8 = 1 << 5;
    const QOS_MASK: u8 = 1 << 4 | 1 << 3;
    const WILL_FLAG_MASK: u8 = 1 << 2;
    const CLEAN_START_MASK: u8 = 1 << 1;
}

impl From<ConnectFlags> for u8 {
    fn from(value: ConnectFlags) -> Self {
        let flags = u8::from(value.username) << 7 | u8::from(value.password) << 6 | u8::from(value.will_retain) << 5 | u8::from(value.will_qos) << 4 | 
        u8::from(value.will_flag) << 2 | u8::from(value.clean_start) << 1;
        flags
    }
}

impl TryFrom<u8> for ConnectFlags {
    type Error = MQTTError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let username = (value & Self::USERNAME_MASK) != 0;
        let password = (value & Self::PASSWORD_MASK) != 0;
        let will_retain = (value & Self::WILL_RETAIN_MASK) != 0;
        let will_qos = QoS::try_from((value & Self::QOS_MASK) >> 3)?;
        let will_flag = (value & Self::WILL_FLAG_MASK) != 0;
        let clean_start = (value & Self::CLEAN_START_MASK) != 0;

        Ok(Self { username, password, will_retain, will_qos, will_flag, clean_start })
    }
}

#[derive(Debug, Length)]
pub struct Connect {
    version: Version,
    client_id: String,
    #[bytes(ignore)]
    will: Option<Will>,
    username: Option<String>,
    password: Option<String>,

    #[bytes(ignore)]
    clean_start: bool,
    #[bytes(ignore)]
    keep_alive: u16,
    #[bytes(ignore)] // Connection properties
    conn_ppts: ConnectProperties,
}

impl BufferIO for Connect {
    fn length(&self) -> usize {
        let mut len = (2 + PROTOCOL_NAME.len()) + 1 + 1 + 2; // version + connect flags + keep alive
        len += self.conn_ppts.length();
        len += variable_length(self.conn_ppts.length());
        if let Some(will) = &self.will { len += will.length() }
        len += self.len(); // client id + username + password

        len
    }

    fn w(&self, buf: &mut bytes::BytesMut) {
        buf.put_u8(Packet::Connect.into());
        let _ = variable_integer(buf, self.length());
        self.ws(buf, PROTOCOL_NAME.as_bytes());
        buf.put_u8(self.version as u8);
        
        let mut flags = ConnectFlags {
            clean_start: self.clean_start,
            password: self.password.is_some(),
            username: self.username.is_some(),
            ..Default::default()
        };

        if let Some(will) = &self.will {
            flags.will_retain = will.retain;
            flags.will_flag = true;
            flags.will_qos = will.qos;
        }
        
        buf.put_u8(u8::from(flags));
        buf.put_u16(self.keep_alive);
        self.conn_ppts.w(buf);
        self.ws(buf, self.client_id.as_bytes());
        if let Some(will) = &self.will { will.w(buf) }
        if let Some(username) = &self.username { self.ws(buf, username.as_bytes()) }
        if let Some(password) = &self.password { self.ws(buf, password.as_bytes()) }   
    }
}


/// CONNECT Properties (3.1.2.11)
#[derive(Debug, Clone, Length)]
pub(crate) struct ConnectProperties {
    session_expiry_interval: Option<u32>,
    receive_maximum: Option<u16>,
    maximum_packet_size: Option<u32>,
    topic_alias_maximum: Option<u16>,
    request_response_information: Option<u8>,
    request_problem_information: Option<u8>,
    user_property: Vec<(String, String)>,
    authentication_method: Option<String>,
    authentication_data: Option<Bytes>,
}


impl BufferIO for ConnectProperties {
    fn w(&self, buf: &mut bytes::BytesMut) {
        let _ = variable_integer(buf, self.len()).unwrap(); // from the DataSize macro (3.1.2.11.1)
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReceiveMaximum(self.receive_maximum).w(buf);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buf);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buf);
        Property::RequestResponseInformation(self.request_response_information).w(buf);
        Property::RequestProblemInformation(self.request_problem_information).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        Property::AuthenticationMethod(self.authentication_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.authentication_data.as_deref().map(Cow::Borrowed)).w(buf);
    }

    fn length(&self) -> usize {
        self.len()
    }
}


#[derive(Length, Debug, Clone)]
pub(crate) struct WillProperties {
    delay_interval: Option<u32>,
    payload_format_indicator: Option<u8>,
    message_expiry_interval: Option<u32>,
    content_type: Option<String>,
    response_topic: Option<String>,
    correlation_data: Option<Bytes>,
    user_property: Vec<(String, String)>,
}


impl BufferIO for WillProperties {
    fn w(&self, buff: &mut bytes::BytesMut) {
        let _ = variable_integer(buff, self.len()).unwrap();
        Property::WillDelayInterval(self.delay_interval).w(buff);
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buff);
        Property::MessageExpiryInterval(self.message_expiry_interval).w(buff);
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buff);
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buff);
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buff);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buff));
    }

    fn length(&self) -> usize {
        self.len()
    }
}


#[derive(Debug, Length)]
pub(crate) struct Will {
    properties: WillProperties,
    topic: String,
    payload: Bytes,
    #[bytes(ignore)]
    pub(super) qos: QoS,
    #[bytes(ignore)]
    pub(super) retain: bool,
}


impl BufferIO for Will {
    fn w(&self, buff: &mut bytes::BytesMut) {
        self.properties.w(buff);
        self.ws(buff, self.topic.as_bytes());
        self.ws(buff, &self.payload);
    }

    fn length(&self) -> usize {
        let ppts = self.properties.length();
        self.len() + variable_length(ppts) + ppts
    }
}