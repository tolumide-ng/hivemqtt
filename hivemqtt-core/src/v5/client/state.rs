use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use bytes::Bytes;
#[cfg(feature = "logs")]
use tracing::error;

use crate::v5::{commons::{error::MQTTError, qos::QoS}, packet::{connack::ConnAck, publish::Publish, subscribe::Subscribe, unsubscribe::UnSubscribe}};

use super::{packet_id::PacketIdManager, ConnectOptions};


#[derive(Debug, Default)]
struct TopicAlias {
    outgoing: Arc<Mutex<HashMap<u16, String>>>,
    incoming: Arc<Mutex<HashMap<u16, String>>>,
}

pub(crate) struct State {
    // pkid_mgr: PacketIdManager,
    /// HashMap(alias --> topic)
    topic_alias_max: u16,
    topic_aliases: TopicAlias,
    /// QoS 1 and 2 publish packets that haven't been acked yet
    outgoing_pub: Arc<Mutex<Vec<Option<Publish>>>>,
    /// received QoS 2 PacketIDs
    incoming_pub: HashSet<u16>,
    /// PacketIDs of QoS2 send publish packets
    outgoing_rel: HashSet<u16>,
    /// Manually or Automatically acknolwedge pubs/subs
    manual_ack: bool,
    outgoing_sub: Arc<Mutex<HashSet<u16>>>,
    outgoing_unsub: Arc<Mutex<HashSet<u16>>>,
    clean_start: bool,
}

impl State {
    pub(crate) fn new(options: ConnectOptions) -> Self {
        Self {
            // pkid_mgr: PacketIdManager::new(options.send_max),
            topic_aliases: TopicAlias::default(),
            outgoing_pub: Arc::new(Mutex::new(Vec::with_capacity(options.send_max.into()))),
            incoming_pub: HashSet::new(),
            outgoing_rel: HashSet::new(),
            outgoing_sub: Arc::new(Mutex::new(HashSet::new())),
            outgoing_unsub: Arc::new(Mutex::new(HashSet::new())),
            manual_ack: false,
            topic_alias_max: options.topic_alias_max,
            clean_start: options.clean_start,
        }
    }

    fn handle_incoming_connack(&self, packet: &ConnAck) {}

    fn handle_outgoing_subscribe(&self, packet: &Subscribe) {
        let new_pkid = self.outgoing_sub.lock().unwrap().insert(packet.packet_identifier);
        #[cfg(feature = "logs")]
        if !new_pkid {
            error!("Duplicate Packet ID: {}", packet.packet_identifier);
        }
    }

    fn handle_outgoing_unsubscribe(&self, packet: &UnSubscribe) {
        let new_pkids = self.outgoing_unsub.lock().unwrap().insert(packet.packet_identifier);
        #[cfg(feature = "logs")]
        if !new_pkids {
            error!("Duplicate Packet ID: {}", packet.packet_identifier);
        }
    }

    fn handle_outgoing_publish(&self, packet: &Publish) -> Result<(), MQTTError> {
        if packet.qos != QoS::Zero {
            let pid = packet.packet_identifier.unwrap();
            let index = (pid - 1) as usize;
            if self.outgoing_pub.lock().unwrap()[index].is_some() {
                error!("Duplicate Packet ID: {}", pid);
                return Err(MQTTError::PacketIdConflict(pid))
            }
        }

        if let Some(alias) = &packet.properties.topic_alias {
            if alias == &0 || alias > &self.topic_alias_max {
                return Err(MQTTError::InvalidProperty(
                    format!("Topic Alias Must be non-zero and less than or equal to topic alias maximum {} but got {:?}", self.topic_alias_max, alias)
                ))   
            }
            self.topic_aliases.outgoing.lock().unwrap().insert(*alias, packet.topic.to_owned());
        }

        Ok(())
    }
}