mod properties;
mod will;


use bytes::Bytes;
use hivemqtt_macros::Length;
pub use properties::ConnectProperties;
use will::Will;

use crate::{commons::{error::MQTTError, fixed_header::FixedHeader, packets::Packet, property::Property, qos::QoS, version::Version}, constants::PROTOCOL_NAME, traits::bufferio::BufferIO};
use crate::traits::{write::Write, read::Read};

#[derive(Debug, Length)]
pub struct Connect {
    #[bytes(no_id)]
    pub client_id: String,
    #[bytes(no_id)]
    pub username: Option<String>,
    #[bytes(no_id)]
    pub password: Option<String>,
    #[bytes(ignore)]
    pub version: Version,
    #[bytes(ignore)]
    pub will: Option<Will>,
    #[bytes(ignore)]
    pub clean_start: bool,
    #[bytes(ignore)]
    pub keep_alive: u16,
    #[bytes(ignore)] // Connection properties
    pub properties: ConnectProperties,
}

impl Default for Connect {
    fn default() -> Self {
        Self { 
            client_id: "HiveMQTT".into(), 
            username: None, 
            password: None, 
            version: Version::V5, 
            will: None, 
            clean_start: true, 
            keep_alive: 0, 
            properties: ConnectProperties::default()
        }
    }
}


impl BufferIO for Connect {
    /// Length of the Variable Header + the length of the Payload
    fn length(&self) -> usize {
        let mut len: usize = (2 + PROTOCOL_NAME.len()) + 1 + 1 + 2; // version + connect flags + keep alive
        
        len += self.properties.length();
        len += self.properties.variable_length();
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
        self.properties.write(buf)?; // 3.1.2.11
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
        packet.properties = ConnectProperties::read(buf)?;
        packet.client_id = String::read(buf)?;
        
        if flags.will_flag {
            let mut will = Will::read(buf)?;
            will.retain = flags.will_retain;
            will.qos = flags.will_qos;
            packet.clean_start = flags.will_retain;
            packet.will = Some(will); 
        }

        if flags.username { packet.username = Some(String::read(buf)?) };
        if flags.password { packet.password = Some(String::read(buf)?) };


        Ok(packet)
    }
}


#[derive(Debug, Default, Clone, Copy)]
pub struct ConnectFlags {
    pub username: bool,
    pub password: bool,
    pub will_retain: bool,
    pub will_qos: QoS,
    pub will_flag: bool,
    pub clean_start: bool,
}


impl From<ConnectFlags> for u8 {
    fn from(value: ConnectFlags) -> Self {
        let flags = u8::from(value.username) << 7 | u8::from(value.password) << 6 | u8::from(value.will_retain) << 5 | u8::from(value.will_qos) << 3 |
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
        let will_qos = QoS::try_from((value & (0b11 << 3)) >> 3)?;
        let will_flag = (value & (1 << 2)) != 0;
        let clean_start = (value & (1 << 1)) != 0;

        Ok(Self { username, password, will_retain, will_qos, will_flag, clean_start })
    }
}

#[cfg(test)]
mod connect_packet {
    use std::io::Read;
    use bytes::BytesMut;
    use crate::commons::qos::QoS;
    use super::*;


    #[cfg(test)]
    mod write {
        use will::WillProperties;

        use super::*;
        #[test]
        fn create_connect_packet() -> Result<(), MQTTError> {
            let mut buf = BytesMut::new();
    
            Connect::default().write(&mut buf)?;
            let expected = b"\x10\x15\0\x04MQTT\x05\x02\0\0\0\0\x08HiveMQTT".as_ref().to_vec();
            let received = buf.bytes().flatten().collect::<Vec<u8>>();
            assert_eq!(expected, received);
    
    
            let mut connect = Connect::default();
            connect.username = Some("username".into());
            connect.password = Some("password".into());
            connect.keep_alive = 170;
            connect.will = Some(Will::default());
            let will = connect.will.as_mut().unwrap();
            will.topic = String::from("auto_warmup");
            will.qos = QoS::Two;
            will.properties = WillProperties::default();
            will.payload = b"will payload".to_vec().into();
    
            let mut buf = BytesMut::new();
            connect.write(&mut buf)?;

            let expected = b"\x10E\0\x04MQTT\x05\xd6\0\xaa\0\0\x08HiveMQTT\0\0\x0bauto_warmup\0\x0cwill payload\0\x08username\0\x08password".as_ref().to_vec();
            let received = buf.bytes().flatten().collect::<Vec<u8>>();
    
            assert_eq!(expected, received);
            Ok(())
        }
    }


    #[cfg(test)]
    mod read {
        use super::*;

        #[test]
        fn read_connect_packet() {
            let mut input = b"\0\x04MQTT\x05\xd6\0\xaa\0\0\x08HiveMQTT\0\0\x0bauto_warmup\0\x0cwill payload\0\x08username\0\x08password".as_ref().into();
            let packet = Connect::read(&mut input).unwrap();

            assert_eq!(packet.username.unwrap(), "username".to_string());
            assert_eq!(packet.password.unwrap(), "password".to_string());
            let will = packet.will.unwrap();
            assert_eq!(will.qos, QoS::Two);
            assert_eq!(will.retain, false);

            assert_eq!(will.topic, "auto_warmup");
            assert_eq!(packet.keep_alive, 170);
            assert_eq!(will.payload, b"will payload".to_vec());
            assert_eq!(will.retain, false);

            assert_eq!(packet.version, Version::V5);
            assert_eq!(packet.client_id, "HiveMQTT");
            assert_eq!(packet.clean_start, false);
            assert_eq!(packet.properties.authentication_data, None);
            assert_eq!(packet.properties.authentication_method, None);
            assert_eq!(packet.properties.receive_maximum, None);
        }

    }


}