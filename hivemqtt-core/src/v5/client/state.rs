use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use bytes::Bytes;
#[cfg(feature = "logs")]
use tracing::error;

use crate::v5::packet::{connack::ConnAck, publish::Publish, subscribe::Subscribe};

use super::{packet_id::PacketIdManager, ConnectOptions};

pub(crate) struct State {
    pkid_mgr: PacketIdManager,
    // HashMap(alias --> topic)
    topic_aliases: HashMap<u16, Bytes>,
    topic_alias_max: u16,
    /// QoS 1 and 2 publish packets that haven't been acked yet
    outgoing_pub: Vec<Option<Publish>>,
    /// received QoS 2 PacketIDs
    incoming_pub: HashSet<u16>,
    /// PacketIDs of QoS2 send publish packets
    outgoing_rel: HashSet<u16>,
    /// Manually or Automatically acknolwedge pubs/subs
    manual_ack: bool,
    outgoing_sub: Arc<Mutex<HashSet<u16>>>,
    outgoing_unsub: HashSet<u16>,
    clean_start: bool,
}

impl State {
    pub(crate) fn new(options: ConnectOptions) -> Self {
        Self {
            pkid_mgr: PacketIdManager::new(options.send_max),
            topic_aliases: HashMap::new(),
            outgoing_pub: Vec::with_capacity(options.send_max.into()),
            incoming_pub: HashSet::new(),
            outgoing_rel: HashSet::new(),
            outgoing_sub: Arc::new(Mutex::new(HashSet::new())),
            outgoing_unsub: HashSet::new(),
            manual_ack: false,
            topic_alias_max: options.topic_alias_max,
            clean_start: options.clean_start,
        }
    }

    fn handle_incoming_connack(&self, packet: &ConnAck) {}

    fn handle_outgoing_subscribe(&self, packet: &Subscribe) {
        if let Ok(mut val) = self.outgoing_sub.lock() {
            if val.insert(packet.packet_identifier) {
                #[cfg(feature = "logs")]
                error!("Blah blah blah {}", packet.packet_identifier);
            }
        }
    }
}