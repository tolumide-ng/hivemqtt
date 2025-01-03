use std::sync::{Arc, Mutex, OnceLock};

use crate::v5::client::packet_id::PacketIdManager;

pub(crate) const MAX: usize = 65_535;
pub(crate) const PROTOCOL_NAME: &'static str = "MQTT";

// this need to be updated in such a way that we can easily create newer instances for newer clients
pub(crate) static PID: OnceLock<Arc<Mutex<PacketIdManager>>> = OnceLock::new();