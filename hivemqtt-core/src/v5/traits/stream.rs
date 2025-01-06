use futures::StreamExt;

use crate::v5::{client::ConnectOptions, commons::{error::MQTTError, packet::Packet}, packet::connack::ConnAck};

use super::bufferio::BufferIO;

pub trait AsyncStreamExt<T>: Unpin + Sized + Send + Sync + StreamExt<Item = T> {
    fn read(&mut self) -> impl std::future::Future<Output = Result<Packet, MQTTError>> + Send + Sync;

    // fn write(&mut self, packet: Packet) -> impl std::future::Future<Output = Result<(), MQTTError>> + Send + Sync;

    // fn flush(&mut self) -> impl std::future::Future<Output = Result<(), MQTTError>> + Send + Sync;

    // fn write_many(&mut self, packet: &[Packet]) -> impl std::future::Future<Output = Result<(), MQTTError>> + Send + Sync;

    // fn connect(&mut self, options: &ConnectOptions) -> impl std::future::Future<Output = Result<ConnAck, MQTTError>> + Send + Sync;
}


impl<S, T> AsyncStreamExt<T> for S
    where S: Unpin + Send + Sync + Sized + StreamExt<Item = T>
{
    async fn read(&mut self) -> Result<Packet, MQTTError> {
        Packet::read(self)
    }
}