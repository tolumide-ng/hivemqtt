#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum QoS {
    #[default]
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

impl From<u8> for QoS {
    fn from(value: u8) -> Self {
        match value {
            0 => QoS::Zero,
            1 => QoS::One,
            2 => QoS::Two,
            _ => unimplemented!("Unrecognized QoS {}", value)
        }
    }
}