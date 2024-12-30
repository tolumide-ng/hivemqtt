pub mod network;
pub mod stream;
pub(crate) mod state;
pub(crate) mod packet_id;


pub struct ConnectOptions {
    /// Highest value a client will accept as a topic aloas sent by the server
    topic_alias_max: u16,
    /// Used to limit the number of QoS 1 and QoS 2 publications that can be processed concurrently
    receive_max: u16,
    /// Maximum number of QoS1 and QoS2packets from this client to the broker
    send_max: u16,
    clean_start: bool,
    max_packet_size: u32, // all of these would be changed for a direct reference to the connect packet in the future
    
}