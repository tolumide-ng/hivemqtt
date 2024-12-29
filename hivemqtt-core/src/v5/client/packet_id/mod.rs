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
    const BITS: usize = usize::BITS as usize;

    pub(crate) fn new(max_packets: u16) -> Self {
        let num_shards = (max_packets as usize + Self::BITS - 1) / Self::BITS;
        Self { 
            shards: Vec::with_capacity(num_shards),
            semaphore: Arc::new(Semaphore::new(max_packets.into()))
        }
    }

    #[cfg(not(feature = "sync"))]
    pub(crate) async fn allocate(&self) -> Option<u16> {
        let _permit = self.semaphore.acquire().await.ok();

        for (shard_index, shard) in self.shards.iter().enumerate() {
            if let Some(id) = shard.allocate() {
                let packet_id = shard_index * (Self::BITS) + id as usize;
                return Some(packet_id as u16);
            }
        }

        None
    }

    #[cfg(feature = "sync")]
    pub(crate) fn allocate_sync (&self) -> Option<u16> {
        let _permit = self.semaphore.try_acquire().ok()?;
        for (shard_index, shard) in self.shards.iter().enumerate() {
            if let Some(id) = shard.allocate() {
                let packet_id = shard_index * (Self::BITS) + id as usize;
                return Some(packet_id as u16);
            }
        }
        None
    }

    pub(crate) fn release(&self, id: u16) {
        let shard_index = (id as usize) / Self::BITS;
        let actual_index_in_shard = ((id as usize) % Self::BITS) as u8;
        self.shards.get(shard_index).and_then(|shard| Some(shard.release(actual_index_in_shard)));
        // if shard_index >= self.shards.len() {}
        // self.shards[shard_index].release(actual_index_in_shard);
        
        // Releases the semaphore slot
        self.semaphore.add_permits(1);
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn creates_a_packetid_manager() {}

    #[test]
    fn can_allocate_a_packet_id() {}

    #[test]
    fn can_release_a_packet_id() {}

    #[test]
    fn must_not_panick_if_packet_id_is_out_of_bounds() {}
}