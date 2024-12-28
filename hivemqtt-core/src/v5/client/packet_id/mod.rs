use std::sync::Arc;

#[cfg(feature = "tokio")]
use tokio::sync::Semaphore;
#[cfg(feature = "snmol")]
use smol::lock::Semaphore;

mod shard;
use shard::PacketIdShard;

/// 0x3FF = 1023 (on a 64-bit architecture)
const SHARD_LENGTH: usize = (u16::MAX as usize) / (usize::BITS as usize);

pub(crate) struct PacketIdManager {
    shards: [PacketIdShard; SHARD_LENGTH],
    semaphore: Arc<Semaphore>
}