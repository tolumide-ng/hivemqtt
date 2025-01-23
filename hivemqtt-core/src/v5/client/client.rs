use async_channel::Sender;

use crate::v5::commons::packet::Packet;

#[derive(Debug)]
pub struct MqttClient {
    tx: Sender<Packet>,
}

impl MqttClient {
    pub(crate) fn new(tx: Sender<Packet>) -> Self {
        
        Self { tx }
    }
}
