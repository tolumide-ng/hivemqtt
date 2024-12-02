#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum Version {
    V4 = 0b0000_0100,
    V5 = 0b0000_0101
}