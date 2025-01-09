use std::borrow::Cow;

use futures::AsyncWriteExt;

pub(crate) use crate::v5::commons::property::syncx::Property;
use crate::v5::{commons::error::MQTTError, traits::asyncx::write::Write};


#[cfg(feature = "async")]
impl<'a> Property<'a> {
    async fn with_ida<S, F>(&self, stream: &mut S, func: F) -> Result<(), MQTTError>
        where F: Fn(&mut S),
            S: AsyncWriteExt + Unpin {
                u8::from(self).write(stream).await?;
                func(stream);
                Ok(())
    }
}

// impl<'a> BufferIO for Property<'a> {
//     fn length(&self) -> usize {
//         match self {
//             Self::SubscriptionIdentifier(length) => **length,
//             _ => 0
//         }
//     }
//     fn w(&self, buf: &mut BytesMut) {
//         match self {
//             Self::SessionExpiryInterval(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::ReceiveMaximum(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::MaximumPacketSize(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::TopicAliasMaximum(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::TopicAlias(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::RequestResponseInformation(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::RequestProblemInformation(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::UserProperty(Cow::Borrowed((ref k, ref v))) => self.with_id(buf, |b| { k.write(b); v.write(b); }),
//             Self::AuthenticationMethod(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
//             Self::AuthenticationData(Some(p)) => self.with_id(buf, |b| Bytes::from_iter(p.to_vec()).write(b)),
//             Self::WillDelayInterval(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::PayloadFormatIndicator(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::MessageExpiryInterval(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::ContentType(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
//             Self::ResponseTopic(Some(p)) => self.with_id(buf, |b| Bytes::from_iter(p.as_bytes().to_vec()).write(b)),
//             Self::CorrelationData(Some(p)) => self.with_id(buf, |b| Bytes::from_iter(p.to_vec()).write(b)),
//             // NOTE: this needs to be tested for if this method of writing is correct or not!
//             Self::SubscriptionIdentifier(_) => self.with_id(buf, |b| { self.encode(b).unwrap() }),
//             Self::AssignedClientIdentifier(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
//             Self::ServerKeepAlive(Some(p)) => self.with_id(buf, |b| p.write(b)),
//             Self::ResponseInformation(Some(data)) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
//             Self::ServerReference(Some(data )) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
//             Self::ReasonString(Some(data )) => self.with_id(buf, |b| Bytes::from_iter(data.as_bytes().to_vec()).write(b)),
//             Self::MaximumQoS(Some(i)) => self.with_id(buf, |b| i.write(b)),
//             Self::RetainAvailable(Some(i)) => self.with_id(buf, |b| i.write(b)),
//             Self::WildCardSubscription(Some(i)) => self.with_id(buf, |b| i.write(b)),
//             Self::SubscriptionIdentifierAvailable(Some(i)) => self.with_id(buf, |b| i.write(b)),
//             Self::SharedSubscriptionAvailable(Some(i)) => self.with_id(buf, |b| i.write(b)),
//             // _ => {unreachable!("Unrecognized enum variant or argument!")}
//             _ => {}
//         }
//     }


//     fn read(buf: &mut Bytes) -> Result<Self, MQTTError> {
//         if buf.is_empty() { return Err(MQTTError::IncompleteData("MQTT Property", 1, 0))}

//         match buf.get_u8() {
//             1  =>  Ok(Property::PayloadFormatIndicator(Some(u8::read(buf)?))),
//             2  =>  Ok(Property::MessageExpiryInterval(Some(u32::read(buf)?))),
//             3  =>  Ok(Property::ContentType(Some(Cow::Owned(String::read(buf)?)))),
//             8  =>  Ok(Property::ResponseTopic(Some(Cow::Owned(String::read(buf)?)))),
//             9  =>  Ok(Property::CorrelationData(Some(Cow::Owned(Bytes::read(buf)?.to_vec())))),
//             11 =>  Ok(Property::SubscriptionIdentifier(Cow::Owned(Self::decode(buf)?.0))),
//             17 =>  Ok(Property::SessionExpiryInterval(Some(u32::read(buf)?))),
//             18 =>  Ok(Property::AssignedClientIdentifier(Some(Cow::Owned(String::read(buf)?)))),
//             19 =>  Ok(Property::ServerKeepAlive(Some(u16::read(buf)?))),
//             21 =>  Ok(Property::AuthenticationMethod(Some(Cow::Owned(String::read(buf)?)))),
//             22 =>  Ok(Property::AuthenticationData(Some(Cow::Owned((Bytes::read(buf)?).to_vec())))),
//             23 =>  Ok(Property::RequestProblemInformation(Some(u8::read(buf)?))),
//             24 =>  Ok(Property::WillDelayInterval(Some(u32::read(buf)?))),
//             25 =>  Ok(Property::RequestResponseInformation(Some(u8::read(buf)?))),
//             26 =>  Ok(Property::ResponseInformation(Some(Cow::Owned(String::read(buf)?)))),
//             28 =>  Ok(Property::ServerReference(Some(Cow::Owned(String::read(buf)?)))),
//             31 =>  Ok(Property::ReasonString(Some(Cow::Owned(String::read(buf)?)))),
//             33 =>  Ok(Property::ReceiveMaximum(Some(u16::read(buf)?))),
//             34 =>  Ok(Property::TopicAliasMaximum(Some(u16::read(buf)?))),
//             35 =>  Ok(Property::TopicAlias(Some(u16::read(buf)?))),
//             36 =>  Ok(Property::MaximumQoS(Some(u8::read(buf)?))),
//             37 =>  Ok(Property::RetainAvailable(Some(u8::read(buf)?))),
//             38 =>  Ok(Property::UserProperty(Cow::Owned((String::read(buf)?, String::read(buf)?)))),
//             39 =>  Ok(Property::MaximumPacketSize(Some(u32::read(buf)?))),
//             40 =>  Ok(Property::WildCardSubscription(Some(u8::read(buf)?))),
//             41 =>  Ok(Property::SubscriptionIdentifierAvailable(Some(u8::read(buf)?))),
//             42 =>  Ok(Property::SharedSubscriptionAvailable(Some(u8::read(buf)?))),
//             v => Err(MQTTError::UnknownProperty(v))
//         }
//     }
// }



// /// this would eventually be changed to use derive_more lib
// impl<'a> Display for Property<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "this would be changed eventually to use derive_more::Error")
//     }
// }