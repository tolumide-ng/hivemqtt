/// The Connect Flags bytes provides information on the MQTT connection, 
/// and indicates the presence or absence of fields in the payload 
pub struct ConnectFlags {
    username: bool,
    password: bool,
    will_retain: bool,
    will_flag: bool,
    clean_start: bool
}

impl ConnectFlags {
    pub fn new(username: bool, password: bool, will_retain: bool, will_flag: bool, clean_start: bool) {}
}

impl From<ConnectFlags> for u8 {
    fn from(value: ConnectFlags) -> Self {
        let mut flags = 0;
        if value.clean_start { flags |= 0x2 }
        if value.will_flag { flags |= 0x4 }
        if value.will_retain { flags |= 0x4 }
        // if value.p { flags |= 0x4 }
        if value.will_flag { flags |= 0x4 }
        
        flags
    }
}