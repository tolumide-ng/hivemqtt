use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use async_channel::Receiver;
use futures::{pin_mut, select, AsyncReadExt, AsyncWriteExt, FutureExt};
use smol::pin;

use crate::v5::{
    client::{client::MqttClient, handler::AsyncHandler, state::State, ConnectOptions},
    commons::{error::MQTTError, packet::Packet},
    packet::{
        connack::{reason_code::ConnAckReasonCode, ConnAck},
        connect::Connect,
        ping::PingReq,
    },
    traits::streamio::StreamIO,
};

use super::PacketIdManager;

#[derive(Debug)]
pub struct Network<S> {
    stream: S,
    options: ConnectOptions,
    client: Option<MqttClient>,
    pkids: PacketIdManager,
    state: State,
    rx: Receiver<Packet>,
}

impl<S> Network<S>
where
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    pub async fn new(options: ConnectOptions, stream: S) -> Result<Self, MQTTError> {
        let pkids = Arc::new(Mutex::new(PacketIdManager::new(0)));
        let state = State::new(&options);

        let (tx, rx) = async_channel::bounded::<Packet>(100); // receive_max + send_max

        let mut network = Self {
            stream,
            options,
            client: None,
            pkids,
            state,
            rx,
        };

        let connack = network.connect().await?;
        let server_receive_max = connack.properties.receive_maximum.unwrap_or(100);
        network.pkids = PacketIdManager::new(server_receive_max);

        network.client = Some(MqttClient::new(tx));

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
            return Ok(connack);
        }

        Err(MQTTError::ConnectionRefused(connack.reason.into()))
    }

    async fn xx() {}

    async fn run<H>(&mut self, handler: &mut H) -> Result<Packet, MQTTError>
    where
        H: AsyncHandler,
    {
        let mut last_ping = if self.options.keep_alive > 0 {
            Some(Instant::now())
        } else {
            None
        };

        let keep_alive = self.options.keep_alive;
        let mut expecting_pingresp = false;
        let max_timeout = keep_alive as u64 * (3 / 2);

        // let xx = pin_mut!(Box);
        let mut xx = Box::pin(Self::xx().fuse());
        // let xx = pin!(xx);
        // let abc = Box::pin(last_heartbeat);

        let reschedule = || {
            expecting_pingresp = false;
            last_ping = Some(Instant::now());
        };

        // let xxx = self.state.handle;
        loop {
            select! {
                packet = Packet::read(&mut self.stream).fuse() => {
                    let mut data = packet?;
                    expecting_pingresp = false;

                    match data {
                        Packet::PingResp(_) => {
                            handler.handle(data).await;
                            last_ping = Some(Instant::now());
                        }
                        Packet::Disconnect(_) => {
                            handler.handle(data).await;
                            // return Ok(data);
                            break;
                        }
                        _ => {
                            // let xx = self.state.handle
                        }
                    }
                },
                _ = xx => {},
                // receiving incoming packets
                // sending outgoing packets
                // pinging
                // ponging
                 default => {
                     let Some(last_time) = last_ping else { continue; };
                     let since = last_time.elapsed().as_secs();
                    if expecting_pingresp {
                        if since >= max_timeout {
                            return Err(MQTTError::TimeoutError);
                        }
                        continue;
                    }

                    if last_ping.is_some_and(|t| t.elapsed().as_secs() >= keep_alive as u64) {
                        Packet::PingReq(PingReq::default()).write(&mut self.stream).await?;
                        last_ping = Some(Instant::now());
                        expecting_pingresp = true;
                    }
                },
            };
        }

        Err(MQTTError::ConnectionError)
    }
}
