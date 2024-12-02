use bytes::{BufMut, BytesMut};

pub(crate) trait Write {
    fn w(&self, buff: &mut BytesMut);

    /// Writes the length of the bytes and itself into the buffer
    fn ws(&self, buff: &mut BytesMut, value: &[u8]) {
        buff.put_u16(value.len() as u16);
        buff.extend_from_slice(value);
    }


    /// Writes the vairable header into the buffer (write_variable_header)
    fn w_vh(&self, buff: &mut BytesMut) {}
}