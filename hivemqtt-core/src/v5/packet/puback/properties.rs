use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::{FromU8, Length};

use crate::v5::{
    commons::error::MQTTError,
    traits::{read_data::ReadData, utils::Utils},
};

use super::{BufferIO, Property};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, FromU8, Default)]
pub enum PubAckReasonCode {
    #[default]
    Success = 0,
    NoMatchingSubscribers = 16,
    UnspecifiedError = 128,
    ImplementationSpecificError = 131,
    NotAuthorized = 135,
    TopicNameInvalid = 144,
    PacketIdentifierInUse = 145,
    QuotaExceeded = 151,
    PayloadFormatInvalid = 153,
}

#[derive(Debug, Length, Default, PartialEq, Eq)]
pub struct PubAckProperties {
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

impl ReadData for PubAckProperties {
    fn read_data(data: &mut Bytes) -> Result<Self, MQTTError> {
        let mut props = Self::default();

        loop {
            let property = Property::read(data)?;

            match property {
                Property::ReasonString(ref v) => Self::try_update(
                    &mut props.reason_string,
                    v.as_deref().map(String::from),
                )(property)?,
                Property::UserProperty(value) => props.user_property.push(value.into_owned()),
                p => return Err(MQTTError::UnexpectedProperty(p.to_string(), "".to_string())),
            }

            if data.is_empty() {
                break;
            }
        }

        Ok(props)
    }
}

mod syncx {
    use std::borrow::Cow;

    use crate::v5::{
        commons::{error::MQTTError, property::Property},
        traits::{bufferio::BufferIO, read_data::ReadData},
    };

    use super::PubAckProperties;

    impl BufferIO for PubAckProperties {
        fn length(&self) -> usize {
            self.len()
        }

        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            self.encode(buf)?;

            Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).write(buf)?;
            self.user_property
                .iter()
                .try_for_each(|kv| Property::UserProperty(Cow::Borrowed(kv)).write(buf))?;
            Ok(())
        }

        fn read(buf: &mut bytes::Bytes) -> Result<Self, MQTTError> {
            let Some(len) = Self::parse_len(buf)? else {
                return Ok(Self::default());
            };

            let mut data = buf.split_to(len);

            Self::read_data(&mut data)
        }
    }
}

mod asyncx {
    use std::borrow::Cow;

    use bytes::Bytes;

    use crate::v5::{
        commons::{error::MQTTError, property::Property},
        traits::{read_data::ReadData, streamio::StreamIO},
    };

    use super::PubAckProperties;

    impl StreamIO for PubAckProperties {
        fn length(&self) -> usize {
            self.len()
        }

        async fn write<W>(&self, stream: &mut W) -> Result<(), MQTTError>
        where
            W: futures::AsyncWriteExt + Unpin,
        {
            self.encode(stream).await?;

            Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed))
                .write(stream)
                .await?;
            for kv in &self.user_property {
                Property::UserProperty(Cow::Borrowed(kv))
                    .write(stream)
                    .await?;
            }

            Ok(())
        }

        async fn read<R>(stream: &mut R) -> Result<Self, MQTTError>
        where
            R: futures::AsyncReadExt + Unpin,
        {
            let Some(len) = Self::parse_len(stream).await? else {
                return Ok(Self::default());
            };

            let mut data = Vec::with_capacity(len);
            stream.read_exact(&mut data).await?;
            let mut data = Bytes::copy_from_slice(&data);

            Self::read_data(&mut data)
        }
    }
}
