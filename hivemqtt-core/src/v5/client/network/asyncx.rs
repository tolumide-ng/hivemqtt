use std::marker::PhantomData;

use futures::{AsyncReadExt, AsyncWriteExt};

use crate::v5::{
    client::{client::MqttClient, handler::AsyncHandler, ConnectOptions},
    commons::{error::MQTTError, packet::Packet},
    packet::{
        connack::{reason_code::ConnAckReasonCode, ConnAck},
        connect::Connect,
    },
    traits::streamio::StreamIO,
};

use super::PacketIdManager;

#[derive(Debug)]
pub struct Network<H, S> {
    stream: S,
    handler: H,
    options: ConnectOptions,
    client: MqttClient,
    pkids: PacketIdManager,
}

impl<H, S> Network<H, S>
where
    H: AsyncHandler,
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    pub async fn new(options: ConnectOptions, stream: S, handler: H) -> Result<Self, MQTTError> {
        let pkids = PacketIdManager::new(0);
        let client = MqttClient::new();

        let mut network = Self {
            stream,
            handler,
            options,
            client,
            pkids,
        };

        network.connect().await?;

        Ok(network)
    }

    async fn connect(&mut self) -> Result<ConnAck, MQTTError> {
        Packet::Connect(Connect::from(&self.options))
            .write(&mut self.stream)
            .await?;

        let packet = Packet::read(&mut self.stream).await?;

        let Packet::ConnAck(connack) = packet else {
            return Err(MQTTError::ConnectionError); // this needs to be return an Error that contains the packet received
        };

        if connack.reason == ConnAckReasonCode::Success {
            self.pkids = PacketIdManager::new(connack.properties.receive_maximum.unwrap_or(100));
            return Ok(connack);
        }

        Err(MQTTError::ConnectionRefused(connack.reason.into()))
    }
}
