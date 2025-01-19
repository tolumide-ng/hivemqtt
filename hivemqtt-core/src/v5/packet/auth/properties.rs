use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::{
    commons::{error::MQTTError, property::Property},
    traits::update::try_update,
};

use hivemqtt_macros::FromU8;

#[derive(Debug, Default, Length, PartialEq, Eq)]
pub struct AuthProperties {
    pub auth_method: Option<String>,
    pub auth_data: Option<Bytes>,
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

impl AuthProperties {
    pub(crate) fn read_data(data: &mut Bytes) -> Result<Self, MQTTError> {
        let mut props = Self::default();

        loop {
            let property = Property::read(data)?;
            match property {
                Property::AuthenticationMethod(ref v) => {
                    try_update(&mut props.auth_method, v.as_deref().map(String::from))(property)?
                }
                Property::AuthenticationData(ref value) => try_update(
                    &mut props.auth_data,
                    value.as_deref().map(|x| Bytes::from_iter(x.to_vec())),
                )(property)?,
                Property::ReasonString(ref v) => {
                    try_update(&mut props.reason_string, v.as_deref().map(String::from))(property)?
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromU8)]
pub enum AuthReasonCode {
    #[default]
    Success = 0,
    ContinueAuthentication = 24,
    ReAuthenticate = 25,
}

#[cfg(not(feature = "asyncx"))]
mod syncx {
    use std::borrow::Cow;

    use bytes::Bytes;

    use super::{AuthProperties, Property};
    use crate::v5::{commons::error::MQTTError, traits::bufferio::BufferIO};

    impl BufferIO for AuthProperties {
        fn length(&self) -> usize {
            self.len()
        }

        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            self.encode(buf)?;

            Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed))
                .write(buf)?;
            Property::AuthenticationData(self.auth_data.as_deref().map(Cow::Borrowed))
                .write(buf)?;
            Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).write(buf)?;
            self.user_property
                .iter()
                .try_for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).write(buf))?;
            Ok(())
        }

        fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
            let Some(len) = Self::parse_len(buf)? else {
                return Ok(Self::default());
            };

            let mut data = buf.split_to(len);

            Self::read_data(&mut data)
        }
    }
}

#[cfg(feature = "asyncx")]
mod asyncx {
    use std::borrow::Cow;

    use crate::v5::commons::error::MQTTError;
    use crate::v5::packet::auth::AuthProperties;
    use crate::v5::traits::streamio::StreamIO;
    use bytes::Bytes;

    use super::Property;

    impl StreamIO for AuthProperties {
        fn length(&self) -> usize {
            self.len()
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

        async fn write<W>(&self, stream: &mut W) -> Result<(), MQTTError>
        where
            W: futures::AsyncWriteExt + Unpin,
        {
            self.encode(stream).await?;

            Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed))
                .write(stream)
                .await?;
            Property::AuthenticationData(self.auth_data.as_deref().map(Cow::Borrowed))
                .write(stream)
                .await?;
            Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed))
                .write(stream)
                .await?;

            for up in &self.user_property {
                Property::UserProperty(Cow::Borrowed(up))
                    .write(stream)
                    .await?
            }

            Ok(())
        }
    }
}
