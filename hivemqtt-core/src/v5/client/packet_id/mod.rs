use std::sync::Arc;

#[cfg(feature = "tokio")]
use tokio::sync::Semaphore;
#[cfg(feature = "snmol")]
use smol::lock::Semaphore;

mod shard;
use shard::PacketIdShard;


pub(crate) struct PacketIdManager {
    shards: Vec<PacketIdShard>,
    semaphore: Arc<Semaphore>
}

impl PacketIdManager {
    pub(crate) fn new(max_packets: u16) -> Self {
        let num_shards = (max_packets as usize + usize::BITS as usize - 1) / usize::BITS as usize;
        Self { 
            shards: Vec::with_capacity(num_shards),
            semaphore: Arc::new(Semaphore::new(2))
        }
    }

    pub(crate) fn allocate(&self) {}
}