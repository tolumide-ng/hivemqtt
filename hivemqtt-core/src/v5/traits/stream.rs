// use std::io::Result;

use async_trait::async_trait;

use crate::v5::{client::ConnectOptions, commons::{error::MQTTError, packet::Packet}, packet::connack::ConnAck};

#[async_trait]
pub(crate) trait AsyncStreamExt {
    async fn read(&mut self) -> impl std::future::Future<Output = Result<Packet, MQTTError>> + Send + Sync;

    fn write(&mut self, packet: Packet) -> impl std::future::Future<Output = Result<(), MQTTError>> + Send + Sync;

    fn flush(&mut self) -> impl std::future::Future<Output = Result<(), MQTTError>> + Send + Sync;

    fn write_many(&mut self, packet: &[Packet]) -> impl std::future::Future<Output = Result<(), MQTTError>> + Send + Sync;

    fn connect(&mut self, options: &ConnectOptions) -> impl std::future::Future<Output = Result<ConnAck, MQTTError>> + Send + Sync;
}
