#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum Version {
    V3,
    V5
}