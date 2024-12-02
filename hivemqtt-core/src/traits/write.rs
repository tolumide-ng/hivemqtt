use bytes::{BufMut, BytesMut};

pub(crate) trait Write {
    fn w(&self, buf: &mut BytesMut);

    /// Writes the length of the bytes and itself into the buffer
    fn ws(&self, buf: &mut BytesMut, value: &[u8]) {
        buf.put_u16(value.len() as u16);
        buf.extend_from_slice(value);
    }
}