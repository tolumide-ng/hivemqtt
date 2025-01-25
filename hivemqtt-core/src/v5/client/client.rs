use std::sync::Arc;

use async_channel::Sender;

use crate::v5::{commons::packet::Packet, traits::pkid_mgr::PacketIdAlloc};

#[derive(Debug)]
pub struct MqttClient<T> {
    tx: Sender<Packet>,
    pkid_alloc: Arc<T>,
}

impl<T> MqttClient<T>
where
    T: PacketIdAlloc,
{
    pub(crate) fn new(tx: Sender<Packet>, pkid_alloc: Arc<T>) -> Self {
        Self { tx, pkid_alloc }
    }

    pub(crate) fn talking(&self) {
        // self.pkid_alloc.allocate();
    }
}
