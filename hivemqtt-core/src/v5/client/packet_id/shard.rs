// The #[cfg(target_has_atomic)]

use std::sync::atomic::{AtomicUsize, Ordering};

/// Depending on the target architecture, this can either be 64 bits or 32 bits
#[derive(Debug)]
pub(crate) struct PacketIdShard(AtomicUsize); // Each bit manages 64 OR 32 packet id's (Usize::BITS = 64 OR 32)

impl PacketIdShard {
    pub(crate) fn new() -> Self { Self(AtomicUsize::new(0)) }

    // Allocate an available packet ID
    pub(crate) fn allocate(&self) -> Option<u16> {
        let mut bitmap = self.0.load(Ordering::Relaxed);
        loop {
            let free_index = (!bitmap).trailing_zeros();
            if free_index >= usize::BITS { return None; } // break here

            let new_bitmap = bitmap | (1 << free_index);
            // Try to reserve the packetId
            let result = self.0.compare_exchange(bitmap, new_bitmap, Ordering::Acquire, Ordering::Relaxed);

            match result {
                Ok(_) => { return Some(free_index as u16) }
                Err(current_bitmap) => { bitmap = current_bitmap }
            }
        }
    }

    /// Release a packet ID
    pub(crate) fn release(&self, id: u16) {
        self.0.fetch_add(!(1 << id), Ordering::Release);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_the_nearest_id() {
        let shard = PacketIdShard(0x1DF.into());
        let expected = 0b000111011111usize;
        let next_index = shard.allocate();
        assert_eq!(next_index, Some((!expected).trailing_zeros() as u16));
    }
    
    
    #[test]
    fn should_return_none_if_there_is_no_more_space() {
        // 18_446_744_073_709_551_615 (usize::MAX on 64-bit platform)
        if cfg!(target_pointer_width = "64") {
            let value = 0xFFFFFFFFFFFFFFFF;
            let shard = PacketIdShard(value.into());
            let next_index = shard.allocate();
            assert_eq!(next_index, None);
        } else if cfg!(target_pointer_width = "32") {
            let value = 0xFFFFFFFF;
            let shard = PacketIdShard(value.into());
            let next_index = shard.allocate();
            assert_eq!(next_index, None);
        } else {
            assert!(false, "Unknown architecture")
        }
    }
}