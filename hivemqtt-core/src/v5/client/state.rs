use std::sync::{Arc, Mutex};

// #[cfg(feature = "logs")]
// use tracing::error;

use crate::v5::{
    commons::{error::MQTTError, packet::Packet, packet_type::PacketType, qos::QoS},
    packet::{
        disconnect::Disconnect,
        puback::PubAck,
        pubcomp::{PubComp, PubCompReasonCode},
        publish::Publish,
        pubrec::{properties::PubRecReasonCode, PubRec},
        pubrel::PubRel,
        suback::SubAck,
        subscribe::Subscribe,
        unsuback::UnSubAck,
        unsubscribe::UnSubscribe,
    },
    traits::pkid_mgr::PacketIdRelease,
    utils::topic::parse_alias,
};

use super::ConnectOptions;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    InBound,
    OutBound,
}

#[derive(Debug, Default)]
struct TopicAlias {
    outgoing: Mutex<Vec<Option<String>>>,
    incoming: Mutex<Vec<Option<String>>>,
}

#[derive(Debug)]
struct ActivePkids {
    /// all pkids generated by the server and received by us(client)
    server: Mutex<Vec<Option<PacketType>>>,
    /// all pkids generated by us (the client), and sent to the server
    client: Mutex<Vec<Option<PacketType>>>,
    unacked_publish: Mutex<Vec<Option<Publish>>>,
}

// Todo!!: All hashsets needs to be changed to vec/VecDeque to improve performance/caching?
#[derive(Debug)]
pub(crate) struct State<T> {
    inbound_topic_alias_max: u16,
    outbound_topic_alias_max: u16,
    topic_aliases: TopicAlias,

    /// Manually or Automatically acknolwedge pubs/subs - this should be removed eventually?
    manual_ack: bool,
    clean_start: bool,
    pub(crate) pkid_mgr: Option<Arc<T>>,

    active_packets: ActivePkids,
}

impl<T> From<&ConnectOptions> for State<T>
where
    T: PacketIdRelease,
{
    fn from(value: &ConnectOptions) -> Self {
        let outgoing_max = value.server_receive_max.get() as usize;
        let incoming_max = value.client_receive_max.get() as usize;

        Self {
            topic_aliases: TopicAlias {
                outgoing: Mutex::new(vec![None; value.outbound_topic_alias_max as usize]),
                incoming: Mutex::new(vec![None; value.outbound_topic_alias_max as usize]),
            },

            active_packets: ActivePkids {
                server: Mutex::new(vec![None; incoming_max]),
                client: Mutex::new(vec![None; outgoing_max]),
                unacked_publish: Mutex::new(vec![None; outgoing_max]),
            },

            manual_ack: value.manual_ack,
            inbound_topic_alias_max: value.inbound_topic_alias_max,
            outbound_topic_alias_max: value.outbound_topic_alias_max,
            clean_start: value.clean_start,
            pkid_mgr: None,
        }
    }
}

impl<T> State<T>
where
    T: PacketIdRelease,
{
    fn parse_topic_and_try_update(
        &self,
        packet: &Publish,
        direction: Direction,
    ) -> Result<String, MQTTError> {
        let topic = &packet.topic;
        let alias = packet.properties.topic_alias;

        let (max, mut record) = if direction == Direction::InBound {
            (
                self.inbound_topic_alias_max,
                self.topic_aliases.incoming.lock().unwrap(),
            )
        } else {
            (
                self.outbound_topic_alias_max,
                self.topic_aliases.outgoing.lock().unwrap(),
            )
        };

        match (topic, alias) {
            (topic, None) if topic.len() > 0 => Ok(topic.to_owned()),
            (topic, Some(alias)) if topic.len() > 0 => {
                let alias = parse_alias(alias, max)?;
                record.insert(alias as usize, Some(topic.clone()));
                Ok(topic.to_owned())
            }
            (topic, Some(alias)) if topic.len() == 0 => {
                let alias = parse_alias(alias, max)?;
                let value = record[alias as usize].clone();
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
            if self.pkid_mgr.as_ref().unwrap().is_occupied(pid) {
                return Err(MQTTError::PacketIdConflict(pid));
            }
        }

        self.parse_topic_and_try_update(&packet, Direction::OutBound)?;

        if packet.qos != QoS::Zero {
            self.active_packets.client.lock().unwrap()[packet.pkid.unwrap() as usize]
                .replace(PacketType::Publish);
            self.active_packets.unacked_publish.lock().unwrap()[packet.pkid.unwrap() as usize]
                .replace(packet);
        }

        Ok(())
    }

    fn handle_incoming_publish(&self, packet: &mut Publish) -> Result<Option<Packet>, MQTTError> {
        let topic = self.parse_topic_and_try_update(&packet, Direction::InBound)?;
        packet.topic = topic;

        if let Some(pid) = packet.pkid {
            if self.active_packets.server.lock().unwrap()[pid as usize].is_some() {
                return Err(MQTTError::PacketIdConflict(pid));
            }
        }

        if packet.qos == QoS::Zero {
            return Ok(None);
        }

        let pkid = packet.pkid.unwrap();
        if packet.qos == QoS::Two && !self.manual_ack {
            self.active_packets.server.lock().unwrap()[pkid as usize] = Some(PacketType::PubRec);
        }

        let result = match (packet.qos, self.manual_ack) {
            // After it has sent a PUBACK packet the receiver MUST treat any incoming PUBLISH packet that contains the same Packet Identifier as being a new Application Message, irrespective of the setting of its DUP flag
            (QoS::One, false) => Some(Packet::PubAck(PubAck {
                pkid,
                ..Default::default()
            })),
            (QoS::Two, false) => Some(Packet::PubRec(PubRec {
                pkid,
                ..Default::default()
            })),
            _ => None,
        };

        Ok(result)
    }

    // we don't need to confirm anything locally
    pub(crate) fn handle_outgoing_puback(&self, p: PubAck) -> Result<(), MQTTError> {
        Ok(())
    }

    pub(crate) fn handle_incoming_puback(
        &self,
        packet: &PubAck,
    ) -> Result<Option<Packet>, MQTTError> {
        let pkid = packet.pkid as usize;

        // release the pkid, and remove the packet from the state
        let prev =
            self.active_packets.client.lock().unwrap()[pkid].take_if(|p| *p == PacketType::Publish);

        if prev.is_none() {
            return Err(MQTTError::UnknownData(format!(
                "Unknown Pakcet Id: {}",
                packet.pkid
            )));
        }

        self.pkid_mgr.as_ref().unwrap().release(packet.pkid);
        self.active_packets.unacked_publish.lock().unwrap()[pkid] = None;

        return Ok(None);
    }

    // we don't need to confirm anything locally
    pub(crate) fn handle_outgoing_pubrec(&self, packet: PubRec) -> Result<(), MQTTError> {
        self.active_packets.server.lock().unwrap()[packet.pkid as usize] = Some(PacketType::PubRec);
        Ok(())
    }

    pub(crate) fn handle_incoming_pubrec(
        &self,
        packet: &PubRec,
    ) -> Result<Option<Packet>, MQTTError> {
        let pkid = packet.pkid as usize;

        match self.active_packets.client.lock().unwrap().get_mut(pkid) {
            Some(pt) if *pt == Some(PacketType::Publish) => {
                // MUST NOT re-send the PUBLISH once it has sent the corresponding PUBREL packet [MQTT-4.3.3-6].
                self.active_packets.client.lock().unwrap()[pkid] = None;
                if packet.reason_code == PubRecReasonCode::NoMatchingSubscribers
                    || packet.reason_code == PubRecReasonCode::Success
                {
                    *pt = Some(PacketType::PubRel);
                } else {
                    *pt = None;
                    return Ok(None);
                }
            }
            _ => {
                return Err(MQTTError::UnknownData(format!(
                    "Unknown pubrec Pakcet Id: {}",
                    packet.pkid
                )));
            }
        };

        if self.manual_ack {
            return Ok(None);
        }

        // MUST treat the PUBREL packet as “unacknowledged” until it has received the corresponding PUBCOMP packet from the receiver [MQTT-4.3.3-5].
        return Ok(Some(Packet::PubRel(PubRel {
            pkid: packet.pkid,
            ..Default::default()
        })));
    }

    // we don't need to confirm anything locally, if it's autohandled good, if it's not, then it's up to the user
    pub(crate) fn handle_outgoing_pubrel(&self, packet: PubRel) -> Result<(), MQTTError> {
        self.active_packets.client.lock().unwrap()[packet.pkid as usize] = Some(PacketType::PubRel);
        Ok(())
    }

    pub(crate) fn handle_incoming_pubrel(
        &self,
        packet: &PubRel,
    ) -> Result<Option<Packet>, MQTTError> {
        let pkid = packet.pkid as usize;

        let prev = self.active_packets.server.lock().unwrap()[pkid]
            .take_if(|pt| *pt == PacketType::PubRec);

        if self.manual_ack {
            return Ok(None);
        }

        if prev.is_none() {
            return Ok(Some(Packet::PubComp(PubComp {
                pkid: packet.pkid,
                reason_code: PubCompReasonCode::PacketIdentifierNotFound,
                ..Default::default()
            })));
        }

        Ok(Some(Packet::PubComp(PubComp {
            pkid: packet.pkid,
            ..Default::default()
        })))
    }

    fn handle_outgoing_pubcomp(&self, _packet: PubComp) -> Result<(), MQTTError> {
        Ok(())
    }

    fn handle_incoming_pubcomp(&self, packet: &PubComp) -> Result<Option<Packet>, MQTTError> {
        let pkid = packet.pkid as usize;
        let prev = self.active_packets.client.lock().unwrap()[pkid]
            .take_if(|pt| *pt == PacketType::PubRel);

        if prev.is_none() {
            return Err(MQTTError::UnknownData(format!(
                "Unknown Packet Id: {}",
                packet.pkid
            )));
        }

        self.pkid_mgr.as_ref().unwrap().release(packet.pkid);

        return Ok(None);
    }

    fn handle_outgoing_subscribe(&self, packet: Subscribe) -> Result<(), MQTTError> {
        self.active_packets.client.lock().unwrap()[packet.pkid as usize] =
            Some(PacketType::Subscribe);

        Ok(())
    }

    fn handle_incoming_suback(&self, packet: &SubAck) -> Result<Option<Packet>, MQTTError> {
        let prev = self.active_packets.client.lock().unwrap()[packet.pkid as usize]
            .take_if(|pt| *pt == PacketType::Subscribe);

        if prev.is_none() {
            return Err(MQTTError::UnknownData(format!(
                "Unknown Suback Packet Id: {}",
                packet.pkid
            )));
        }

        self.pkid_mgr.as_ref().unwrap().release(packet.pkid);
        return Ok(None);
    }

    fn handle_outgoing_unsubscribe(&self, packet: UnSubscribe) -> Result<(), MQTTError> {
        self.active_packets.client.lock().unwrap()[packet.pkid as usize] =
            Some(PacketType::UnSubscribe);

        Ok(())
    }

    fn handle_incoming_unsuback(&self, packet: &UnSubAck) -> Result<Option<Packet>, MQTTError> {
        let prev = self.active_packets.client.lock().unwrap()[packet.pkid as usize];
        if prev.is_none() {
            return Err(MQTTError::UnknownData(format!(
                "Unknown Suback Packet Id: {}",
                packet.pkid
            )));
        }

        self.pkid_mgr.as_ref().unwrap().release(packet.pkid);
        return Ok(None);
    }

    fn handle_outgoing_disconnect(&self, _packet: Disconnect) -> Result<(), MQTTError> {
        Ok(())
    }

    pub(crate) fn handle_incoming_packet(
        &self,
        packet: &mut Packet,
    ) -> Result<Option<Packet>, MQTTError> {
        match packet {
            Packet::Publish(packet) => self.handle_incoming_publish(packet),
            Packet::PubAck(packet) => self.handle_incoming_puback(packet),
            Packet::PubRec(packet) => self.handle_incoming_pubrec(packet),
            Packet::PubRel(packet) => self.handle_incoming_pubrel(packet),
            Packet::PubComp(packet) => self.handle_incoming_pubcomp(packet),
            Packet::SubAck(packet) => self.handle_incoming_suback(packet),
            Packet::UnSubAck(packet) => self.handle_incoming_unsuback(packet),
            _ => Err(MQTTError::UnsupportedQoS(0)),
        }
    }

    pub(crate) fn handle_outgoing_packet(&self, packet: Packet) -> Result<(), MQTTError> {
        match packet {
            Packet::Publish(packet) => self.handle_outgoing_publish(packet),
            Packet::PubAck(packet) => self.handle_outgoing_puback(packet),
            Packet::PubRec(packet) => self.handle_outgoing_pubrec(packet),
            Packet::PubRel(packet) => self.handle_outgoing_pubrel(packet),
            Packet::PubComp(packet) => self.handle_outgoing_pubcomp(packet),
            Packet::Subscribe(packet) => self.handle_outgoing_subscribe(packet),
            Packet::UnSubscribe(packet) => self.handle_outgoing_unsubscribe(packet),
            Packet::Disconnect(packet) => self.handle_outgoing_disconnect(packet),

            _ => Err(MQTTError::UnsupportedQoS(0)),
        }
    }

    /// should be used in the case of clean_start=0,
    /// dup flag on publish packets must be set to 1 in this case
    /// see: 4.3.3 QoS 2: Exactly once delivery
    /// for more information
    pub(crate) fn retransmit_all() {}
}
