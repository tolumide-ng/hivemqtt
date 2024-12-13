use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property, qos::QoS, version::Version}, constants::PROTOCOL_NAME, traits::bufferio::BufferIO};
use crate::traits::{write::Write, read::Read};

#[derive(Debug, Length, Default)]
pub struct Connect {
    #[bytes(no_id)]
    client_id: String,
    #[bytes(no_id)]
    username: Option<String>,
    #[bytes(no_id)]
    password: Option<String>,
    
    #[bytes(ignore)]
    version: Version,
    #[bytes(ignore)]
    will: Option<Will>,
    #[bytes(ignore)]
    clean_start: bool,
    #[bytes(ignore)]
    keep_alive: u16,
    #[bytes(ignore)] // Connection properties
    conn_ppts: ConnectProperties,
}


impl BufferIO for Connect {
    /// Length of the Variable Header + the length of the Payload
    fn length(&self) -> usize {
        let mut len: usize = (2 + PROTOCOL_NAME.len()) + 1 + 1 + 2; // version + connect flags + keep alive
        
        len += self.conn_ppts.length();
        len += self.conn_ppts.variable_length();
        if let Some(will) = &self.will { len += will.length() }
        len += self.len(); // client id + username + password

        len
    }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        FixedHeader::new(Packet::Connect, 0, self.length()).write(buf)?;
        
        (PROTOCOL_NAME.to_string()).write(buf);
        (self.version as u8).write(buf);

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

        u8::from(flags).write(buf); // 3.1.2.3
        self.keep_alive.write(buf); // 3.1.2.10
        self.conn_ppts.write(buf)?; // 3.1.2.11
        // CONNECT Payload: length-prefixed fields
        self.client_id.write(buf); // ClientId, willProperties, willTopic, willPayload, userName, password
        if let Some(will) = &self.will { will.write(buf)?; }
        if let Some(username) = &self.username { username.write(buf); } // 3.1.3.5
        if let Some(password) = &self.password { password.write(buf); } // 3.1.3.6

        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        // Assumption is that the fixed header as been read already
        String::read(buf).and_then(|x| { if x == "MQTT".to_string() { return Ok(x)} return Err(MQTTError::MalformedPacket)})?;

        let mut packet = Self::default();
        packet.version = Version::try_from(u8::read(buf)?)?;


        let flags = ConnectFlags::try_from(u8::read(buf)?)?;
        packet.keep_alive = u16::read(buf)?;
        packet.conn_ppts = ConnectProperties::read(buf)?;
        packet.client_id = String::read(buf)?;
        
        if flags.will_flag {
            let mut will = Will::read(buf)?;
            will.retain = flags.will_retain;
            will.qos = flags.will_qos;
            packet.will = Some(will); 
        }

        if flags.username { packet.username = Some(String::read(buf)?) };
        if flags.password { packet.password = Some(String::read(buf)?) };


        Ok(packet)
    }


}


/// CONNECT Properties (3.1.2.11)
#[derive(Debug, Clone, Length, Default)]
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
    /// The length of the Properties in the CONNECT packet Variable Header encoded as a Variable Byte Integer 3.1.2.11.1
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?; // 3.1.2.11.1 (Property Length)
        Property::SessionExpiryInterval(self.session_expiry_interval).w(buf);
        Property::ReceiveMaximum(self.receive_maximum).w(buf);
        Property::MaximumPacketSize(self.maximum_packet_size).w(buf);
        Property::TopicAliasMaximum(self.topic_alias_maximum).w(buf);
        Property::RequestResponseInformation(self.request_response_information).w(buf);
        Property::RequestProblemInformation(self.request_problem_information).w(buf);
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf));
        Property::AuthenticationMethod(self.authentication_method.as_deref().map(Cow::Borrowed)).w(buf);
        Property::AuthenticationData(self.authentication_data.as_deref().map(Cow::Borrowed)).w(buf);
        Ok(())  
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let len = Self::decode(buf)?;
        let mut properties = Self::default();

        if len == 0 { return Ok(properties) }
        else if len > buf.len() { return Err(MQTTError::IncompleteData("ConnectionProperties", len, buf.len()) )}

        let mut data = buf.split_to(len);

        loop {
            let property = Property::read(&mut data)?;
            match property {
                Property::SessionExpiryInterval(value) => Self::try_update(&mut properties.session_expiry_interval, value)(property)?,
                Property::ReceiveMaximum(value) => Self::try_update(&mut properties.receive_maximum, value)(property)?,
                Property::MaximumPacketSize(value) => { Self::try_update(&mut properties.maximum_packet_size, value)(property)?; },
                Property::TopicAliasMaximum(value) => { Self::try_update(&mut properties.topic_alias_maximum, value)(property)?; },
                Property::RequestResponseInformation(value) => { Self::try_update(&mut properties.request_response_information, value)(property)? }
                Property::RequestProblemInformation(value) => { Self::try_update(&mut properties.request_problem_information, value)(property)? }
                Property::UserProperty(value) => { properties.user_property.push(value.into_owned()); }
                Property::AuthenticationMethod(ref value) => { Self::try_update(&mut properties.authentication_method, value.as_deref().map(|x| String::from(x)))(property)? }
                Property::AuthenticationData(ref value) => {
                    Self::try_update(&mut properties.authentication_data, value.to_owned().map(|x| Bytes::from_iter(x.into_owned())))(property)?
                }
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }
            if data.is_empty() { break; }
        }
        
        Ok(properties)
    }
}


#[derive(Debug, Default)]
pub(crate) struct ConnectFlags {
    pub(super) username: bool,
    pub(super) password: bool,
    pub(super) will_retain: bool,
    pub(super) will_qos: QoS,
    pub(super) will_flag: bool,
    pub(super) clean_start: bool,
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
        let username = (value & (1 << 7)) != 0;
        let password = (value & (1 << 6)) != 0;
        let will_retain = (value & (1 << 5)) != 0;
        let will_qos = QoS::try_from((value & (1 << 4 | 1 << 3)) >> 3)?;
        let will_flag = (value & (1 << 2)) != 0;
        let clean_start = (value & (1 << 1)) != 0;

        Ok(Self { username, password, will_retain, will_qos, will_flag, clean_start })
    }
}



#[derive(Length, Debug, Clone, Default)]
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
    fn length(&self) -> usize { self.len() }

    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.encode(buf)?; // 3.1.3.2.1

        Property::WillDelayInterval(self.delay_interval).w(buf); // 3.1.3.2.2
        Property::PayloadFormatIndicator(self.payload_format_indicator).w(buf); // 3.1.3.2.3
        Property::MessageExpiryInterval(self.message_expiry_interval).w(buf); // 3.1.3.2.4
        Property::ContentType(self.content_type.as_deref().map(Cow::Borrowed)).w(buf); // 3.1.3.2.5
        Property::ResponseTopic(self.response_topic.as_deref().map(Cow::Borrowed)).w(buf); // 3.1.3.2.6
        Property::CorrelationData(self.correlation_data.as_deref().map(Cow::Borrowed)).w(buf); // 3.1.3.2.7
        self.user_property.iter().for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).w(buf)); // 3.1.3.2.8

        Ok(())
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let length = Self::decode(buf)?;
        let mut properties = Self::default();

        if length == 0 { return Ok(properties) }
        else if length > buf.len() { return Err(MQTTError::IncompleteData("WillProperties", length, buf.len()) )};

        let mut data = buf.split_to(length);

        loop {
            let property = Property::read(&mut data)?;
            match property {
                Property::WillDelayInterval(value) => Self::try_update(&mut properties.delay_interval, value)(property)?,
                Property::PayloadFormatIndicator(value) => Self::try_update(&mut properties.payload_format_indicator, value)(property)?,
                Property::MessageExpiryInterval(value) => Self::try_update(&mut properties.message_expiry_interval, value)(property)?,
                Property::ContentType(ref value) => Self::try_update(&mut properties.content_type, value.as_deref().map(|x| String::from(x)))(property)?,
                Property::ResponseTopic(ref value) => Self::try_update(&mut properties.response_topic, value.as_deref().map(|x| String::from(x)))(property)?,
                Property::CorrelationData(ref value) => Self::try_update(&mut properties.correlation_data, value.as_deref().map(|x| Bytes::from_iter(x.to_vec())))(property)?,
                Property::UserProperty(value) => { properties.user_property.push(value.into_owned()); },
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string()))
            }

            if data.is_empty() { break; }
        }

        Ok(properties)
    }
}


#[derive(Debug, Length, Default)]
pub(crate) struct Will {
    #[bytes(ignore)]
    properties: WillProperties,
    #[bytes(no_id)]
    topic: String,
    #[bytes(no_id)]
    payload: Bytes,
    #[bytes(ignore)]
    pub(super) qos: QoS,
    #[bytes(ignore)]
    pub(super) retain: bool,
}


impl BufferIO for Will {
    fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
        self.properties.write(buf)?;
        self.topic.write(buf); // 3.1.3.3
        self.payload.write(buf); // 3.1.3.4
        Ok(())
    }

    fn length(&self) -> usize {
        self.len() + self.properties.variable_length() + self.properties.length()
    }

    fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
        let mut will = Self::default();

        will.properties = WillProperties::read(buf)?;
        will.topic = String::read(buf)?;
        will.payload = Bytes::read(buf)?;
        Ok(will)
    }
}





#[cfg(test)]
mod connect_packet {
    use super::*;

    fn packet () -> [u8; 43] {[
        0x10, 0x2A, 0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x05, 0x02, 0x00, 0x3C, 
        0x00, 0x07, 0x4D, 0x79, 0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x00, 0x0B, 
        0x73, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x2F, 0x74, 0x6F, 0x70, 0x69, 0x63, 
        0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
    ]}

    // const PACKET: [u8; 43] = [
    //     0x10, 0x2A, 0x00, 0x04, 0x4D, 0x51, 0x54, 0x54, 0x05, 0x02, 0x00, 0x3C, 
    //     0x00, 0x07, 0x4D, 0x79, 0x43, 0x6C, 0x69, 0x65, 0x6E, 0x74, 0x00, 0x0B, 
    //     0x73, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x2F, 0x74, 0x6F, 0x70, 0x69, 0x63, 
    //     0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
    // ];


    #[test]
    fn read_connect_packet() {
        let connect = Connect::read(&mut Bytes::from_iter(packet()));
        println!("connect packet is >>>>");
    }

    #[test]
    fn write_connection_properties() {}

    // #[cfg(test)]
    // mod packet_length {}
}