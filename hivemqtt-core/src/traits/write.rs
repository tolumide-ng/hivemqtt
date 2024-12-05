use bytes::{BufMut, BytesMut};

pub(crate) trait ControlPacket {
    fn w(&self, buf: &mut BytesMut);

    /// Writes the length of the bytes and itself into the buffer
    fn ws(&self, buf: &mut BytesMut, value: &[u8]) {
        buf.put_u16(value.len() as u16);
        buf.extend_from_slice(value);
    }

    /// Allows a struct specify what it's length is to it's external users
    /// Normally this is obtainable using the .len() method (internally on structs implementing Length(formerly DataSize)),
    /// However, this method allows the struct customize what its actual length is.
    /// NOTE: The eventual plan is to make this the only property accessible externally and 
    ///     make `.len()` internal while probably enforcing that all struct's implementing this method/trait
    ///     must also implement `DataSize` proc. So that there is a default accurate length property
    fn length(&self) -> usize { 0 }
}