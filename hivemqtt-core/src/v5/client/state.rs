use std::{borrow::{Borrow, Cow}, collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use bytes::Bytes;
#[cfg(feature = "logs")]
use tracing::error;

use crate::v5::{commons::{error::MQTTError, qos::QoS}, packet::{connack::ConnAck, puback::PubAck, publish::Publish, pubrec::PubRec, pubrel::PubRel, subscribe::Subscribe, unsubscribe::UnSubscribe}};

use super::{packet_id::PacketIdManager, ConnectOptions};


#[derive(Debug, Default)]
struct TopicAlias {
    outgoing: Arc<Mutex<HashMap<u16, String>>>,
    incoming: Arc<Mutex<HashMap<u16, String>>>,
}


struct PubData {}

// Todo: All hashsets needs to be changed to vec/VecDeque to improve performance/caching?
pub(crate) struct State {
    // pkid_mgr: PacketIdManager,
    /// HashMap(alias --> topic)
    topic_alias_max: u16,
    topic_aliases: TopicAlias,
    /// QoS 1 and 2 publish packets that haven't been acked yet
    outgoing_pub: Arc<Mutex<HashMap<u16, Publish>>>,
    /// received QoS 2 PacketIDs
    incoming_pub: Arc<Mutex<HashSet<u16>>>,
    /// PacketIDs of QoS2 send publish packets
    outgoing_rel: Arc<Mutex<HashSet<u16>>>,
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
            outgoing_pub: Arc::new(Mutex::new(HashMap::new())),
            incoming_pub: Arc::new(Mutex::new(HashSet::new())),
            outgoing_rel: Arc::new(Mutex::new(HashSet::new())),
            outgoing_sub: Arc::new(Mutex::new(HashSet::new())),
            outgoing_unsub: Arc::new(Mutex::new(HashSet::new())),
            manual_ack: false,
            topic_alias_max: options.topic_alias_max,
            clean_start: options.clean_start,
        }
    }

    fn handle_outgoing_subscribe(&self, packet: Subscribe) {
        let new_pkid = self.outgoing_sub.lock().unwrap().insert(packet.packet_identifier);
        #[cfg(feature = "logs")]
        if !new_pkid {
            error!("Duplicate Packet ID: {}", packet.packet_identifier);
        }
    }

    fn handle_outgoing_unsubscribe(&self, packet: UnSubscribe) {
        let new_pkids = self.outgoing_unsub.lock().unwrap().insert(packet.packet_identifier);
        #[cfg(feature = "logs")]
        if !new_pkids {
            error!("Duplicate Packet ID: {}", packet.packet_identifier);
        }
    }

    fn handle_outgoing_publish(&self, packet: Publish) -> Result<(), MQTTError> {
        // Confirm that the packet identifier is not a duplicate before we proceed with anything
        if packet.qos != QoS::Zero {
            let pid = packet.packet_identifier.unwrap();
            if self.outgoing_pub.lock().unwrap().get(&pid).is_some() {
                return Err(MQTTError::PacketIdConflict(pid))
            }
        }


        let mut topic: Option<String> = None;

        // Confirm that the alias is valid
        if let Some(alias) = &packet.properties.topic_alias {
            if alias == &0 || alias > &self.topic_alias_max {
                return Err(MQTTError::InvalidProperty(
                    format!("Topic Alias Must be non-zero and less than or equal to topic alias maximum {} but got {:?}", self.topic_alias_max, alias)
                ))   
            }

            // check if this alias exists in the aliases if there is no topic length
            topic = self.topic_aliases.outgoing.lock().unwrap().get(&alias).cloned();
            
            let unknown_alias = packet.topic.len() == 0 && topic.is_none();
            if unknown_alias {
                return Err(MQTTError::UnknownData(format!("Unknown Topic Alias {alias}")));
            }

            if packet.topic.len() > 0 {
                self.topic_aliases.outgoing.lock().unwrap().insert(*alias, packet.topic.to_owned());
                topic = Some(packet.topic.clone());
            }


        }


        if packet.qos != QoS::Zero {
            if let Some(k) = self.outgoing_pub.lock().unwrap().get_mut(&packet.packet_identifier.unwrap()) {
                *k = Publish {topic: topic.unwrap(), ..packet};
            }
        }

        Ok(())
    }


    pub(crate) fn handle_outgoing_puback(&self, p: PubAck) -> Result<(), MQTTError> {
        let pkid = p.packet_identifier;
        if self.incoming_pub.lock().unwrap().remove(&pkid) {
            return Ok(())
        }
        Err(MQTTError::PublishPacketId) // returns Error if we have no knowledge of the packet_identifier we're trying to acknowledge
    }

    pub(crate) fn outgoing_pubrec(&mut self, p: PubRec) -> Result<(), MQTTError> {
        let pkid = p.packet_identifier;
        if self.incoming_pub.lock().unwrap().remove(&pkid) {
            return Ok(())
        }
        Err(MQTTError::PublishPacketId) // returns Error if we have no knowledge of the packet_identifier we're trying to acknowledge
    }

    pub(crate) fn outgoing_pubrel(&mut self, p: PubRel) -> Result<(), MQTTError> {
        // the `pkid` must have been added when we received a pubrec, so now we confirm that before sending a `pubrel`
        if self.outgoing_rel.lock().unwrap().contains(&p.packet_identifier) {
            return Ok(())
        }
        Err(MQTTError::UnknownData(format!("{}", p.packet_identifier)))
    }

    // pub(crate) fn incoming_pubrec(&self, p: PubRec) -> Result<(), MQTTError> {
    //     match self.outgoing_pub.lock().unwrap().contains_key(&p.packet_identifier) {
    //         true => {
    //             let unique_id = self.outgoing_rel.lock().unwrap().insert(p.packet_identifier);
    //             if unique_id { return Ok(()) } else { return Err(MQTTError::DuplicateProperty(String::from(""))) }
    //         }
    //         false => {
    //             return Err(MQTTError::UnknownData(format!("Unexpected pubrec: Have no record for publish with id {}", p.packet_identifier)))
    //         }
    //     }
    // }


    fn handle_incoming_connack(&self, packet: &ConnAck) {}
}