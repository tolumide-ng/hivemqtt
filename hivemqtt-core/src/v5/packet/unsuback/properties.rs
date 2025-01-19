use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::{commons::error::MQTTError, traits::update::Utils};

use super::Property;

#[derive(Debug, Default, Length, PartialEq, Eq)]
pub struct UnSubAckProperties {
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

impl UnSubAckProperties {
    fn read_data(data: &mut Bytes) -> Result<Self, MQTTError> {
        let mut props = Self::default();

        loop {
            let property = Property::read(data)?;
            match property {
                Property::ReasonString(ref v) => {
                    Self::try_update(&mut props.reason_string, v.as_deref().map(String::from))(property)?
                }
                Property::UserProperty(v) => props.user_property.push(v.into_owned()),
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
        traits::bufferio::BufferIO,
    };

    use super::UnSubAckProperties;

    impl BufferIO for UnSubAckProperties {
        fn length(&self) -> usize {
            self.len()
        }

        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            self.encode(buf)?;
            Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).write(buf)?;
            self.user_property
                .iter()
                .try_for_each(|up| Property::UserProperty(Cow::Borrowed(up)).write(buf))?;

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

    use crate::v5::{commons::property::Property, traits::streamio::StreamIO};

    use super::UnSubAckProperties;

    impl StreamIO for UnSubAckProperties {
        fn length(&self) -> usize {
            self.len()
        }

        async fn write<W>(&self, stream: &mut W) -> Result<(), crate::v5::commons::error::MQTTError>
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

        async fn read<R>(stream: &mut R) -> Result<Self, crate::v5::commons::error::MQTTError>
        where
            R: futures::AsyncReadExt + Unpin,
            Self: Default,
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
