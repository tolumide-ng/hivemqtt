use bytes::BytesMut;

pub(crate) trait Write {
    fn w(&self, buf: &BytesMut);
}