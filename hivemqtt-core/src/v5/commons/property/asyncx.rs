use futures::{AsyncReadExt, AsyncWriteExt};
use std::borrow::Cow;

pub(crate) use crate::v5::commons::property::syncx::Property;
use crate::v5::{
    commons::error::MQTTError,
    traits::asyncx::{bufferio::BufferIO, read::Read, write::Write},
};

#[cfg(feature = "async")]
impl<'a> Property<'a> {
    async fn write_to_stream<S, T>(&self, stream: &mut S, value: &T) -> Result<(), MQTTError>
    where
        S: AsyncWriteExt + Unpin,
        T: Write<S>,
    {
        u8::from(self).write(stream).await?;
        value.write(stream).await
    }
}

impl<'a, R, W> BufferIO<R, W> for Property<'a>
where
    R: AsyncReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    fn length(&self) -> usize {
        match self {
            Self::SubscriptionIdentifier(length) => **length,
            _ => 0,
        }
    }

    async fn write(&self, stream: &mut W) -> Result<(), MQTTError> {
        match self {
            Self::SessionExpiryInterval(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::ReceiveMaximum(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::MaximumPacketSize(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::TopicAliasMaximum(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::TopicAlias(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::RequestResponseInformation(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::RequestProblemInformation(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::UserProperty(Cow::Borrowed(p)) => self.write_to_stream(stream, *p).await?,
            Self::AuthenticationMethod(Some(p)) => {
                self.write_to_stream(stream, &p.to_string()).await?
            }
            Self::AuthenticationData(Some(p)) => self.write_to_stream(stream, &p.to_vec()).await?,
            Self::WillDelayInterval(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::PayloadFormatIndicator(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::MessageExpiryInterval(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::ContentType(Some(p)) => self.write_to_stream(stream, &p.to_string()).await?,
            Self::ResponseTopic(Some(p)) => self.write_to_stream(stream, &p.to_string()).await?,
            Self::CorrelationData(Some(p)) => self.write_to_stream(stream, &p.to_vec()).await?,
            Self::SubscriptionIdentifier(_) => {
                self.write_to_stream(stream, &(<Self as BufferIO<R, W>>::encode(self).await?))
                    .await?
            }
            Self::AssignedClientIdentifier(Some(p)) => {
                self.write_to_stream(stream, &p.to_string()).await?
            }
            Self::ServerKeepAlive(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::ResponseInformation(Some(p)) => {
                self.write_to_stream(stream, &p.to_string()).await?
            }
            Self::ServerReference(Some(p)) => self.write_to_stream(stream, &p.to_string()).await?,
            Self::ReasonString(Some(p)) => self.write_to_stream(stream, &p.to_string()).await?,
            Self::MaximumQoS(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::RetainAvailable(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::WildCardSubscription(Some(p)) => self.write_to_stream(stream, p).await?,
            Self::SubscriptionIdentifierAvailable(Some(p)) => {
                self.write_to_stream(stream, p).await?
            }
            Self::SharedSubscriptionAvailable(Some(p)) => self.write_to_stream(stream, p).await?,
            _ => (),
        }

        Ok(())
    }

    async fn read(stream: &mut R) -> Result<Self, MQTTError> {
        let property_id = u8::read(stream).await?;

        match property_id {
            1 => Ok(Property::PayloadFormatIndicator(Some(
                u8::read(stream).await?,
            ))),
            2 => Ok(Property::MessageExpiryInterval(Some(
                u32::read(stream).await?,
            ))),
            3 => Ok(Property::ContentType(Some(Cow::Owned(
                String::read(stream).await?,
            )))),
            8 => Ok(Property::CorrelationData(Some(Cow::Owned(
                Vec::read(stream).await?,
            )))),
            11 => Ok(Property::SubscriptionIdentifier(Cow::Owned(
                <Self as BufferIO<R, W>>::decode(stream).await?.0,
            ))),
            17 => Ok(Property::SessionExpiryInterval(Some(
                u32::read(stream).await?,
            ))),
            18 => Ok(Property::AssignedClientIdentifier(Some(Cow::Owned(
                String::read(stream).await?,
            )))),
            19 => Ok(Property::ServerKeepAlive(Some(u16::read(stream).await?))),
            21 => Ok(Property::AuthenticationMethod(Some(Cow::Owned(
                String::read(stream).await?,
            )))),
            22 => Ok(Property::AuthenticationData(Some(Cow::Owned(
                Vec::read(stream).await?,
            )))),
            23 => Ok(Property::RequestProblemInformation(Some(
                u8::read(stream).await?,
            ))),
            24 => Ok(Property::WillDelayInterval(Some(u32::read(stream).await?))),
            25 => Ok(Property::RequestResponseInformation(Some(
                u8::read(stream).await?,
            ))),
            26 => Ok(Property::ResponseInformation(Some(Cow::Owned(
                String::read(stream).await?,
            )))),
            28 => Ok(Property::ServerReference(Some(Cow::Owned(
                String::read(stream).await?,
            )))),
            31 => Ok(Property::ReasonString(Some(Cow::Owned(
                String::read(stream).await?,
            )))),
            33 => Ok(Property::ReceiveMaximum(Some(u16::read(stream).await?))),
            34 => Ok(Property::TopicAliasMaximum(Some(u16::read(stream).await?))),
            35 => Ok(Property::TopicAlias(Some(u16::read(stream).await?))),
            36 => Ok(Property::MaximumQoS(Some(u8::read(stream).await?))),
            37 => Ok(Property::RetainAvailable(Some(u8::read(stream).await?))),
            38 => Ok(Property::UserProperty(Cow::Owned((
                String::read(stream).await?,
                String::read(stream).await?,
            )))),
            39 => Ok(Property::MaximumPacketSize(Some(u32::read(stream).await?))),
            40 => Ok(Property::WildCardSubscription(Some(
                u8::read(stream).await?,
            ))),
            41 => Ok(Property::SubscriptionIdentifierAvailable(Some(
                u8::read(stream).await?,
            ))),
            42 => Ok(Property::SharedSubscriptionAvailable(Some(
                u8::read(stream).await?,
            ))),
            v => Err(MQTTError::UnknownProperty(v)),
        }
    }
}
