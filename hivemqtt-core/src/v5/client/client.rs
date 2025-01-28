use std::sync::Arc;

use async_channel::Sender;

use crate::v5::{commons::packet::Packet, traits::pkid_mgr::PacketIdAlloc};

#[derive(Debug)]
pub struct MqttClient<T> {
    /// sends packets to the channel
    tx: Sender<Packet>,
    pkid_alloc: Arc<T>,
    max_size: usize,
}

impl<T> MqttClient<T>
where
    T: PacketIdAlloc,
{
    pub(crate) fn new(tx: Sender<Packet>, pkid_alloc: Arc<T>, max_size: usize) -> Self {
        Self {
            tx,
            pkid_alloc,
            max_size,
        }
    }

    pub(crate) fn talking(&self) {
        // self.pkid_alloc.allocate();
    }
}

mod asyncx {
    use super::MqttClient;
    use std::fmt::Debug;

    use bytes::Bytes;

    use crate::v5::{
        commons::{error::MQTTError, packet::Packet, qos::QoS},
        packet::publish::Publish,
        traits::{pkid_mgr::PacketIdAlloc, streamio::StreamIO},
    };

    impl<T> MqttClient<T>
    where
        T: PacketIdAlloc,
    {
        pub async fn publish<U, V>(
            &self,
            topic: U,
            qos: QoS,
            retain: bool,
            payload: V,
        ) -> Result<(), MQTTError>
        where
            U: Into<String>,
            V: Into<Bytes>,
        {
            let pkid = match qos {
                QoS::Zero => None,
                _ => Some(
                    self.pkid_alloc
                        .allocate()
                        .ok_or(MQTTError::PacketIdGenerationError)?,
                ),
            };

            let packet = Publish {
                dup: false,
                retain,
                qos,
                topic: topic.into(),
                pkid,
                payload: payload.into(),
                ..Default::default()
            };

            // todo! do topic validation here (check for unnecessary symbols)

            packet.is_valid(self.max_size)?;

            self.tx.send(Packet::Publish(packet)).await;

            Ok(())
        }
    }
}

mod syncx {}
