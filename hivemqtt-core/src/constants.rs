use std::sync::{Arc, Mutex, OnceLock};

use crate::v5::client::packet_id::PacketIdManager;

pub(crate) const MAX: usize = 65_535;
pub(crate) const PROTOCOL_NAME: &'static str = "MQTT";

pub(crate) static PID: OnceLock<Arc<Mutex<PacketIdManager>>> = OnceLock::new();