use crate::commons::qos::QoS;


#[derive(Debug, Default)]
pub(crate) struct ConnectFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_qos: QoS,
    will_flag: bool,
    clean_start: bool,
}

impl ConnectFlags {
    const USERNAME_MASK: u8 = 1 << 7;
    const PASSWORD_MASK: u8 = 1 << 6;
    const WILL_RETAIN_MASK: u8 = 1 << 5;
    const QOS_MASK: u8 = 1 << 4 | 1 << 3;
    const WILL_FLAG_MASK: u8 = 1 << 2;
    const CLEAN_START_MASK: u8 = 1 << 1;
}

impl From<ConnectFlags> for u8 {
    fn from(value: ConnectFlags) -> Self {
        let flags = u8::from(value.username) << 7 | u8::from(value.password) << 6 | u8::from(value.will_retain) << 5 | u8::from(value.will_qos) << 4 | 
        u8::from(value.will_flag) << 2 | u8::from(value.clean_start) << 1;
        flags
    }
}


impl From<u8> for ConnectFlags {
    fn from(value: u8) -> Self {
        let username = (value & Self::USERNAME_MASK) != 0;
        let password = (value & Self::PASSWORD_MASK) != 0;
        let will_retain = (value & Self::WILL_RETAIN_MASK) != 0;
        let will_qos = QoS::from((value & Self::QOS_MASK) >> 3);
        let will_flag = (value & Self::WILL_FLAG_MASK) != 0;
        let clean_start = (value & Self::CLEAN_START_MASK) != 0;
        Self { username, password, will_retain, will_qos, will_flag, clean_start }
    }
}