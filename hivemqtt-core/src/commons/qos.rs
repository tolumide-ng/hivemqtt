use crate::commons::error::MQTTError;

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



impl TryFrom<u8> for QoS {
    type Error = MQTTError;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        type Error = MQTTError;

        match value {
            0 => Ok(QoS::Zero),
            1 => Ok(QoS::One),
            2 => Ok(QoS::Two),
            _ => Err(MQTTError::UnsupportedQoS(value))
        }
    }
}