use std::borrow::Cow;

use bytes::Bytes;
use hivemqtt_macros::Length;

use crate::v5::commons::error::MQTTError;

use super::{BufferIO, Property};

#[derive(Debug, Default, Length, PartialEq, Eq)]
pub struct AuthProperties {
    pub auth_method: Option<String>,
    pub auth_data: Option<Bytes>,
    pub reason_string: Option<String>,
    pub user_property: Vec<(String, String)>,
}

use hivemqtt_macros::FromU8;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, FromU8)]
pub enum AuthReasonCode {
    #[default]
    Success = 0,
    ContinueAuthentication = 24,
    ReAuthenticate = 25,
}

impl AuthProperties {
    fn read_data(data: &mut Bytes) -> Result<Self, MQTTError> {
        let mut props = Self::default();

        loop {
            let property = Property::read(data)?;
            match property {
                Property::AuthenticationMethod(ref v) => Self::try_update(
                    &mut props.auth_method,
                    v.as_deref().map(String::from),
                )(property)?,
                Property::AuthenticationData(ref value) => Self::try_update(
                    &mut props.auth_data,
                    value.as_deref().map(|x| Bytes::from_iter(x.to_vec())),
                )(property)?,
                Property::ReasonString(ref v) => Self::try_update(
                    &mut props.reason_string,
                    v.as_deref().map(String::from),
                )(property)?,
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

    use bytes::Bytes;

    use super::{AuthProperties, BufferIO, Property};
    use crate::v5::commons::error::MQTTError;

    impl BufferIO for AuthProperties {
        fn length(&self) -> usize {
            self.len()
        }

        fn write(&self, buf: &mut bytes::BytesMut) -> Result<(), MQTTError> {
            self.encode(buf)?;

            Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed)).w(buf);
            Property::AuthenticationData(self.auth_data.as_deref().map(Cow::Borrowed)).w(buf);
            Property::ReasonString(self.reason_string.as_deref().map(Cow::Borrowed)).w(buf);
            self.user_property
                .iter()
                .for_each(|up| Property::UserProperty(Cow::Borrowed(&up)).w(buf));
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

mod asynx {
    use std::borrow::Cow;

    use bytes::Bytes;
    use futures::{AsyncReadExt, AsyncWriteExt};

    use crate::v5::{
        commons::{error::MQTTError, property::asyncx::Property},
        traits::asyncx::{bufferio::BufferIO, read::Read, write::Write},
    };

    use super::AuthProperties;

    impl<R, W> BufferIO<R, W> for AuthProperties
    where
        R: AsyncReadExt + Unpin,
        W: AsyncWriteExt + Unpin,
    {
        async fn read(stream: &mut R) -> Result<Self, crate::v5::commons::error::MQTTError> {
            let Some(len) = <Self as BufferIO<R, W>>::parse_len(stream).await? else {
                return Ok(Self::default());
            };

            let mut data = Vec::with_capacity(len);
            stream.read_exact(&mut data).await?;

            let mut data = Bytes::copy_from_slice(&data);

            Self::read_data(&mut data)
        }

        async fn write(&self, stream: &mut W) -> Result<(), MQTTError> {
            let encoded_length = <Self as BufferIO<R, W>>::encode(self).await?;
            encoded_length.write(stream).await?;

            Property::AuthenticationMethod(self.auth_method.as_deref().map(Cow::Borrowed))
                .write::<R, W>(stream);

            Ok(())
        }
    }
}
