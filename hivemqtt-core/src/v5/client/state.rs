use std::{borrow::{Borrow, Cow}, collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

use bytes::Bytes;
#[cfg(feature = "logs")]
use tracing::error;

use crate::v5::{commons::{error::MQTTError, packet::Packet, qos::QoS}, packet::{connack::ConnAck, puback::PubAck, pubcomp::PubComp, publish::Publish, pubrec::{properties::PubRecReasonCode, PubRec}, pubrel::PubRel, subscribe::Subscribe, unsubscribe::UnSubscribe}, utils::topic::parse_alias};

use super::ConnectOptions;


#[derive(Debug, Default)]
struct TopicAlias {
    outgoing: Arc<Mutex<Vec<Option<String>>>>,
    incoming: Arc<Mutex<Vec<Option<String>>>>,
}


struct PubData {
    publish: Arc<Mutex<Vec<Option<Publish>>>>,
    pubrel: Arc<Mutex<Vec<bool>>>
}

// Todo: All hashsets needs to be changed to vec/VecDeque to improve performance/caching?
pub(crate) struct State {
    // pkid_mgr: PacketIdManager,
    /// HashMap(alias --> topic)
    topic_alias_max: u16,
    topic_aliases: TopicAlias,

    /// QoS 1 and 2 publish packets that haven't been acked yet
    outgoing_pub: Arc<Mutex<Vec<Option<Publish>>>>,
    /// PacketIDs of QoS2 send publish packets
    outgoing_rel: Arc<Mutex<Vec<bool>>>,

    /// received QoS 2 PacketIDs
    incoming_pub: Arc<Mutex<Vec<bool>>>,
    /// Manually or Automatically acknolwedge pubs/subs
    manual_ack: bool,
    outgoing_sub: Arc<Mutex<HashSet<u16>>>,
    outgoing_unsub: Arc<Mutex<HashSet<u16>>>,
    clean_start: bool,
}

impl State {
    pub(crate) fn new(options: ConnectOptions) -> Self {
        let receive_max = options.receive_max as usize + 1;
        Self {
            // pkid_mgr: PacketIdManager::new(options.send_max),
            topic_aliases: TopicAlias::default(),
            outgoing_pub: Arc::new(Mutex::new(Vec::with_capacity(receive_max))),
            incoming_pub: Arc::new(Mutex::new(Vec::with_capacity(receive_max))),
            outgoing_rel: Arc::new(Mutex::new(Vec::with_capacity(receive_max))),
            outgoing_sub: Arc::new(Mutex::new(HashSet::new())),
            outgoing_unsub: Arc::new(Mutex::new(HashSet::new())),
            manual_ack: options.manual_ack,
            topic_alias_max: options.topic_alias_max,
            clean_start: options.clean_start,
        }
    }

    // pub(crate) fn handle_incoming_connack(&self, packet: ConnAck) -> Result<(), MQTTError> {
    //     Ok(())
    // }

    fn parse_topic_and_try_update(&self, topic: &String, alias: Option<u16>, aliases: &Arc<Mutex<Vec<Option<String>>>>) -> Result<String, MQTTError> {
        match (topic, alias) {
            (topic, None) if topic.len() > 0 => { Ok(topic.to_owned()) },
            (topic, Some(alias)) if topic.len() > 0 => {
                let alias = parse_alias(alias, self.topic_alias_max)?;
                aliases.lock().unwrap().insert(alias as usize, Some(topic.clone()));
                Ok(topic.to_owned())
            }
            (topic, Some(alias)) if topic.len() == 0 => {
                let alias = parse_alias(alias, self.topic_alias_max)?;
                let value = aliases.lock().unwrap()[alias as usize].clone();
                value.ok_or(MQTTError::UnknownData(format!("Unrecognized Topic Alias {alias}")))
            }
            _ => {Err(MQTTError::UnknownData(format!("Expected Topic or Alias but found none")))}
        }
    }

    fn handle_outgoing_publish(&self, packet: Publish) -> Result<(), MQTTError> {
        // Confirm that the packet identifier is not a duplicate before we proceed with anything
        if let Some(pid) = packet.packet_identifier {
            if self.outgoing_pub.lock().unwrap()[pid as usize].is_some() { return Err(MQTTError::PacketIdConflict(pid)) }
        }

        let topic = self.parse_topic_and_try_update(&packet.topic, packet.properties.topic_alias, &self.topic_aliases.outgoing)?;

        if packet.qos != QoS::Zero {
            self.outgoing_pub.lock().unwrap()[packet.packet_identifier.unwrap() as usize].replace(Publish {topic, ..packet});
        }

        Ok(())
    }

    fn handle_incoming_publish(&self, packet: &mut Publish) -> Result<Option<Packet>, MQTTError> {
        let topic = self.parse_topic_and_try_update(&packet.topic, packet.properties.topic_alias, &self.topic_aliases.incoming)?;
        packet.topic = topic;

        if let Some(pid) = packet.packet_identifier {
            if self.incoming_pub.lock().unwrap()[pid as usize] { return Err(MQTTError::PacketIdConflict(pid)) }
        }

        if packet.qos == QoS::Two { self.incoming_pub.lock().unwrap()[packet.packet_identifier.unwrap() as usize] = true; }
        
        if self.manual_ack || packet.qos == QoS::Zero { return Ok(None) }

        let packet_identifier = packet.packet_identifier.unwrap();
        let result = match packet.qos {
            QoS::One => Some(Packet::PubAck(PubAck {packet_identifier, ..Default::default()})),
            QoS::Two => Some(Packet::PubRec(PubRec {packet_identifier, ..Default::default()})),
            _ => None,
        };
        
        Ok(result)
    }

    pub(crate) fn handle_outgoing_puback(&self, p: PubAck) -> Result<Packet, MQTTError> {
        Ok(Packet::PubRec(PubRec { packet_identifier: p.packet_identifier, ..Default::default() }))
    }

    pub(crate) fn incoming_pubrec(&self, packet: &PubRec) -> Result<Packet, MQTTError> {
        // if self.outgoing_pub.lock().unwrap()[packet.packet_identifier ]
        Err(MQTTError::PacketIdConflict(10))
    }

    // pub(crate) fn outgoing_pubrec(&mut self, p: PubRec) -> Result<(), MQTTError> {
    //     let pkid = p.packet_identifier;
    //     if self.incoming_pub.lock().unwrap().remove(&pkid) {
    //         return Ok(())
    //     }
    //     Err(MQTTError::PublishPacketId) // returns Error if we have no knowledge of the packet_identifier we're trying to acknowledge
    // }

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

    // fn valid_pid(&self, pid: u16) -> Result<(), MQTTError> {
    //     if self.outgoing_pub.lock().unwrap()[pid as usize].is_none() { return Ok(()) }
    //     // Err(MQTTError::PacketIdConflict(pid))
    // }



    pub(crate) fn incoming_pubrec(&self, p: &PubRec) -> Result<Option<PubRel>, MQTTError> {
        let pkid = p.packet_identifier as usize;

        if self.outgoing_pub.lock().unwrap()[pkid].take().is_none() {
            #[cfg(feature = "logs")]
            error!("Unexpected pubrec: Have no record for publish with id {}", pkid);
            return Err(MQTTError::UnknownData(format!("Unexpected pubrec: Have no record for publish with id {}", pkid)))
        }

        if p.reason_code == PubRecReasonCode::Success || p.reason_code == PubRecReasonCode::NoMatchingSubscribers {
            self.outgoing_rel.lock().unwrap()[pkid] = true;
            return Ok(Some(PubRel{packet_identifier: p.packet_identifier, ..Default::default()}))
        }

        Ok(None)
    }

    pub(crate) fn outgoing_pubrel(&self, p: PubRel) -> Result<(), MQTTError> {
        let pkid = p.packet_identifier as usize;
        if self.outgoing_rel.lock().unwrap()[pkid] {
            return Ok(())
        }
        return Err(MQTTError::UnknownData(format!("{}", p.packet_identifier)))
    }

    pub(crate) fn incoming_pubcomp(&self, p: PubComp) -> Result<(), MQTTError> {
        let pkid = p.packet_identifier as usize;
        if self.outgoing_rel.lock().unwrap()[pkid] {
            self.outgoing_rel.lock().unwrap()[pkid] = false;
            return Ok(())
        }
        return Err(MQTTError::UnknownData(format!("Unexpected pubcomp: Have no record for pubrel with id {}", pkid)))
    }
}