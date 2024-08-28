#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum QoS {
    Zero = 0,
    One,
    Two,
}

impl From<QoS> for u8 {
    fn from(value: QoS) -> Self {
        match value {
            QoS::Zero => 0,
            QoS::One => 1,
            QoS::Two => 2,
        }
    }
}