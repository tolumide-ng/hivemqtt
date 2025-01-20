use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc, Mutex,
    },
};

// #[cfg(feature = "logs")]
// use tracing::error;

use crate::v5::{
    commons::{error::MQTTError, packet::Packet, qos::QoS},
    packet::{
        puback::PubAck,
        pubcomp::PubComp,
        publish::Publish,
        pubrec::{properties::PubRecReasonCode, PubRec},
        pubrel::PubRel,
        suback::SubAck,
        subscribe::Subscribe,
    },
    utils::topic::parse_alias,
};

use super::{packet_id::PacketIdManager, ConnectOptions};

#[derive(Debug, Default)]
struct TopicAlias {
    outgoing: Arc<Mutex<Vec<Option<String>>>>,
    incoming: Arc<Mutex<Vec<Option<String>>>>,
}

struct PubData {
    publish: Arc<Mutex<Vec<Option<Publish>>>>,
    pubrel: Arc<Mutex<Vec<bool>>>,
}

// Todo: All hashsets needs to be changed to vec/VecDeque to improve performance/caching?
pub(crate) struct State {
    // pkid_mgr: PacketIdManager,
    /// HashMap(alias --> topic)
    topic_alias_max: u16,
    topic_aliases: TopicAlias,

    /// Most of the outgoing packets might not need to be validated, let the other side side determine it's validity and return appropriate reason code
    /// QoS 1 and 2 publish packets that haven't been acked yet
    /// used to validate incomging pubacks and incoming pubrec
    outgoing_pub: Arc<Mutex<Vec<Option<Publish>>>>,
    /// This is stored immediately a Publish packet is received, and destroyed after PubRel is received
    /// used to validate outgoing pubrec and incoming pubrel
    outgoing_rec: Arc<Mutex<Vec<bool>>>,
    /// This is stored immediately a PubRec packet is received, and destroyed after PubComp is received
    /// used to validate outgoing pubrel and incoming pubcomp
    outgoing_rel: Arc<Mutex<Vec<bool>>>,

    /// Manually or Automatically acknolwedge pubs/subs
    manual_ack: bool,
    outgoing_sub: AtomicU16,
    outgoing_unsub: Arc<Mutex<HashSet<u16>>>,
    clean_start: bool,
    // xx: PacketIdManager
}

impl State {
    pub(crate) fn new(options: ConnectOptions) -> Self {
        let receive_max = options.client_receive_max.get() as usize + 1;
        Self {
            // pkid_mgr: PacketIdManager::new(options.send_max),
            topic_aliases: TopicAlias::default(),

            outgoing_pub: Arc::new(Mutex::new(Vec::with_capacity(receive_max))),
            outgoing_rel: Arc::new(Mutex::new(Vec::with_capacity(receive_max))),
            outgoing_rec: Arc::new(Mutex::new(Vec::with_capacity(receive_max))),

            outgoing_sub: AtomicU16::new(0),
            outgoing_unsub: Arc::new(Mutex::new(HashSet::new())),
            manual_ack: options.manual_ack,
            topic_alias_max: options.topic_alias_max,
            clean_start: options.clean_start,
        }
    }

    fn parse_topic_and_try_update(
        &self,
        topic: &String,
        alias: Option<u16>,
        aliases: &Arc<Mutex<Vec<Option<String>>>>,
    ) -> Result<String, MQTTError> {
        match (topic, alias) {
            (topic, None) if topic.len() > 0 => Ok(topic.to_owned()),
            (topic, Some(alias)) if topic.len() > 0 => {
                let alias = parse_alias(alias, self.topic_alias_max)?;
                aliases
                    .lock()
                    .unwrap()
                    .insert(alias as usize, Some(topic.clone()));
                Ok(topic.to_owned())
            }
            (topic, Some(alias)) if topic.len() == 0 => {
                let alias = parse_alias(alias, self.topic_alias_max)?;
                let value = aliases.lock().unwrap()[alias as usize].clone();
                value.ok_or(MQTTError::UnknownData(format!(
                    "Unrecognized Topic Alias {alias}"
                )))
            }
            _ => Err(MQTTError::UnknownData(format!(
                "Expected Topic or Alias but found none"
            ))),
        }
    }

    fn handle_outgoing_publish(&self, packet: Publish) -> Result<(), MQTTError> {
        // Confirm that the packet identifier is not a duplicate before we proceed with anything
        if let Some(pid) = packet.pkid {
            if self.outgoing_pub.lock().unwrap()[pid as usize].is_some() {
                return Err(MQTTError::PacketIdConflict(pid));
            }
        }

        let topic = self.parse_topic_and_try_update(
            &packet.topic,
            packet.properties.topic_alias,
            &self.topic_aliases.outgoing,
        )?;

        if packet.qos != QoS::Zero {
            self.outgoing_pub.lock().unwrap()[packet.pkid.unwrap() as usize]
                .replace(Publish { topic, ..packet });
        }

        Ok(())
    }

    fn handle_incoming_publish(&self, packet: &mut Publish) -> Result<Option<Packet>, MQTTError> {
        let topic = self.parse_topic_and_try_update(
            &packet.topic,
            packet.properties.topic_alias,
            &self.topic_aliases.incoming,
        )?;
        packet.topic = topic;

        if let Some(pid) = packet.pkid {
            if self.outgoing_rec.lock().unwrap()[pid as usize] {
                return Err(MQTTError::PacketIdConflict(pid));
            }
        }

        if packet.qos == QoS::Two {
            self.outgoing_rec.lock().unwrap()[packet.pkid.unwrap() as usize] = true;
        }

        if self.manual_ack || packet.qos == QoS::Zero {
            return Ok(None);
        }

        let pkid = packet.pkid.unwrap();
        let result = match packet.qos {
            QoS::One => Some(Packet::PubAck(PubAck {
                pkid,
                ..Default::default()
            })),
            QoS::Two => Some(Packet::PubRec(PubRec {
                pkid,
                ..Default::default()
            })),
            _ => None,
        };

        Ok(result)
    }

    pub(crate) fn handle_outgoing_puback(&self, p: PubAck) -> Result<(), MQTTError> {
        Ok(())
    }

    pub(crate) fn handle_incoming_puback(
        &self,
        packet: &PubRec,
    ) -> Result<Option<Packet>, MQTTError> {
        if self.outgoing_pub.lock().unwrap()[packet.pkid as usize]
            .take()
            .is_some()
        {
            return Ok(None);
        }
        Err(MQTTError::UnknownData(format!(
            "Unknown Pakcet Id: {}",
            packet.pkid
        )))
    }

    pub(crate) fn handle_outgoing_pubrec(&self, _packet: PubRec) -> Result<(), MQTTError> {
        Ok(())
    }

    pub(crate) fn handle_incoming_pubrec(&self, p: &PubRec) -> Result<Option<Packet>, MQTTError> {
        let pkid = p.pkid as usize;

        if self.outgoing_pub.lock().unwrap()[pkid].take().is_none() {
            #[cfg(feature = "logs")]
            error!(
                "Unexpected pubrec: Have no record for publish with id {}",
                pkid
            );
            return Err(MQTTError::UnknownData(format!(
                "Unexpected pubrec: Have no record for publish with id {}",
                pkid
            )));
        }

        if p.reason_code == PubRecReasonCode::Success
            || p.reason_code == PubRecReasonCode::NoMatchingSubscribers
        {
            self.outgoing_rel.lock().unwrap()[pkid] = true;
            if !self.manual_ack {
                return Ok(Some(Packet::PubRel(PubRel {
                    pkid: p.pkid,
                    ..Default::default()
                })));
            }
        }

        Ok(None)
    }

    pub(crate) fn handle_outgoing_pubrel(&self, p: PubRel) -> Result<(), MQTTError> {
        Ok(())
    }

    pub(crate) fn handle_incoming_pubrel(&self, p: &PubRel) -> Result<Option<PubComp>, MQTTError> {
        let pkid = p.pkid;
        if self.outgoing_rec.lock().unwrap()[pkid as usize] {
            if self.manual_ack {
                return Ok(Some(PubComp {
                    pkid,
                    ..Default::default()
                }));
            }
            return Ok(None);
        }
        return Err(MQTTError::UnknownData(format!("{}", p.pkid)));
    }

    fn handle_outgoing_pubcomp(&self, p: &PubComp) -> Result<(), MQTTError> {
        Ok(())
    }

    fn handle_incoming_pubcomp(&self, p: &PubComp) -> Result<Option<Packet>, MQTTError> {
        if self.outgoing_rel.lock().unwrap()[p.pkid as usize] {
            return Ok(None);
        }
        return Err(MQTTError::UnknownData(format!("{}", p.pkid)));
    }

    fn handle_outgoing_subscribe(&self, packet: Subscribe) -> Result<(), MQTTError> {
        // we're using the allocate method on pkid, we're confident that the pkid would always be unique
        let pkid = packet.pkid - 1;
        let mask = 1u16 << pkid;

        // this implementatioin is wrong
        if (self.outgoing_sub.load(Ordering::Relaxed) & mask) == 0 {
            return Ok(());
        }
        return Err(MQTTError::PacketIdConflict(packet.pkid));
    }

    fn handle_incoming_suback(&self, packet: SubAck) -> Result<Option<Packet>, MQTTError> {
        let pkid = packet.pkid - 1;
        let mask = 1u16 << pkid;

        // this implementatioin is wrong
        if (self.outgoing_sub.load(Ordering::Relaxed) & mask) != 0 {
            return Err(MQTTError::PacketIdConflict(packet.pkid));
        }
        return Ok(Some(Packet::SubAck(packet)));
    }

    fn handle_outgoing_unsubscribe() {}

    // fn handle_outgoing_unsubscribe(&self, packet: &UnSubscribe) {
    //     let new_pkids = self.outgoing_unsub.lock().unwrap().insert(packet.pkid);
    //     #[cfg(feature = "logs")]
    //     if !new_pkids {
    //         error!("Duplicate Packet ID: {}", packet.pkid);
    //     }
    // }

    // fn valid_pid(&self, pid: u16) -> Result<(), MQTTError> {
    //     if self.outgoing_pub.lock().unwrap()[pid as usize].is_none() { return Ok(()) }
    //     // Err(MQTTError::PacketIdConflict(pid))
    // }

    pub(crate) fn incoming_pubcomp(&self, p: PubComp) -> Result<(), MQTTError> {
        let pkid = p.pkid as usize;
        if self.outgoing_rel.lock().unwrap()[pkid] {
            self.outgoing_rel.lock().unwrap()[pkid] = false;
            return Ok(());
        }
        return Err(MQTTError::UnknownData(format!(
            "Unexpected pubcomp: Have no record for pubrel with id {}",
            pkid
        )));
    }
}
