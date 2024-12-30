use std::collections::HashMap;

use bytes::Bytes;

use crate::v5::packet::publish::Publish;

use super::packet_id::PacketIdManager;

pub(crate) struct State {
    pkid_mgr: PacketIdManager,
    // HashMap(alias --> topic)
    topic_aliases: HashMap<u16, Bytes>,
    outgoing_pub: Vec<Option<Publish>>
}

impl State {
    pub(crate) fn new(outbound_cap: usize, max_packets: u16) -> Self {
        Self {
            pkid_mgr: PacketIdManager::new(max_packets),
            topic_aliases: HashMap::new(),
            outgoing_pub: Vec::with_capacity(outbound_cap)
        }
    }
}