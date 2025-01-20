use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::commons::{error::MQTTError, property::Property};

use super::ReadData;

#[derive(Debug, Length, Default, PartialEq, Eq, Clone)]
pub struct UnSubscribeProperties {
    pub user_property: Vec<(String, String)>,
}

impl ReadData for UnSubscribeProperties {
    fn read_data(data: &mut Bytes) -> Result<Self, MQTTError> {
        let mut props = Self::default();

        loop {
            let property = Property::read(data)?;
            match property {
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
        traits::{bufferio::BufferIO, read_data::ReadData},
    };

    use super::UnSubscribeProperties;

    impl BufferIO for UnSubscribeProperties {
        fn length(&self) -> usize {
            self.len()
        }

        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            self.encode(buf)?;

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
        commons::property::Property,
        traits::{read_data::ReadData, streamio::StreamIO},
    };

    use super::UnSubscribeProperties;

    impl StreamIO for UnSubscribeProperties {
        fn length(&self) -> usize {
            self.len()
        }

        async fn write<W>(&self, stream: &mut W) -> Result<(), crate::v5::commons::error::MQTTError>
        where
            W: futures::AsyncWriteExt + Unpin,
        {
            self.encode(stream).await?;

            for kv in &self.user_property {
                Property::UserProperty(Cow::Borrowed(kv))
                    .write(stream)
                    .await?;
            }

            Ok(())
        }
    }
}
