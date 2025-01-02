pub mod network;
pub mod stream;
pub(crate) mod state;
pub(crate) mod packet_id;


pub struct ConnectOptions {
    /// Highest value a client will accept as a topic aloas sent by the server
    topic_alias_max: u16,
    /// Used to limit the number of QoS 1 and QoS 2 publications that can be processed concurrently
    /// https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901050
    receive_max: u16,
    /// Maximum number of QoS1 and QoS2packets from this client to the broker
    send_max: u16,
    clean_start: bool,
    max_packet_size: u32, // all of these would be changed for a direct reference to the connect packet in the future
    /// Whether the user want's to handle all acks manually, or they want us to do this for them
    manual_ack: bool,

}