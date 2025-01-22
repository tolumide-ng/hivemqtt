use std::{num::NonZero, u32};

use bytes::Bytes;

use super::packet::connect::will::Will;

pub(crate) mod client;
pub mod handler;
pub mod network;
pub(crate) mod packet_id;
pub(crate) mod state;

#[derive(Debug)]
pub struct ConnectOptions {
    /// Whether the user want's to handle all acks manually, or they want us to do this for them
    pub(crate) manual_ack: bool,
    pub(crate) clean_start: bool,
    /// 3.1.2.11.2
    pub(crate) session_expiry_interval: Option<u32>, // default value is 0
    /// 3.1.2.11.4 Maximum number of bytes in an MQTT Control Packet
    pub(crate) server_max_size: NonZero<u32>, // set by connack
    pub(crate) client_max_size: NonZero<u32>, // set by connect

    /// 3.1.2.11.3 The Client uses this value to limit the number of QoS 1 and QoS 2 publications that it is willing to process concurrently
    pub(crate) client_receive_max: NonZero<u16>, // this is us (the client)
    /// 3.2.2.3.3 The Server uses this value to limit the number of QoS 1 and QoS 2 publications that it is willing to process concurrently for the Client.
    /// this determines how much pkids we can make max (the server already told us how much it can handle, so that's the max we can generate)
    /// This is usually received on the CONNACK
    pub(crate) server_receive_max: NonZero<u16>,

    /// 3.1.2.11.5Highest value a client will accept as a topic aloas sent by the server
    pub(crate) topic_alias_max: u16,
    // we use the server's keep_alive from CONNACK else we use the one in CONNECT. Must always be ins econds
    pub(crate) keep_alive: u16,
    pub(crate) will: Option<Will>,
    pub(crate) client_id: String,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,

    // host: Option<String>,
    // port: Option<u16>,
    pub(crate) request_response_information: Option<u8>,
    pub(crate) request_problem_information: Option<u8>,
    pub(crate) user_property: Vec<(String, String)>,
    pub(crate) authentication_method: Option<String>,
    pub(crate) authentication_data: Option<Bytes>,
}

impl Default for ConnectOptions {
    fn default() -> Self {
        Self {
            topic_alias_max: 0,
            manual_ack: false,
            clean_start: true,
            session_expiry_interval: Some(0),
            client_receive_max: NonZero::<u16>::MAX, // connect
            server_receive_max: NonZero::<u16>::MAX, // connack

            keep_alive: 69,
            will: None,
            client_id: String::from("UniqueClientId"),
            username: None,
            password: None,
            // the client uses the size to inform the Server that it will not process packets exceeding this limit
            // WE must return a DISCONNECT with reasoncode 0x95 if we receive any packet larger than this size
            server_max_size: NonZero::<u32>::MAX,
            client_max_size: NonZero::<u32>::MAX,

            request_response_information: None,
            request_problem_information: None,
            user_property: Vec::with_capacity(0),
            authentication_method: None,
            authentication_data: None,
        }
    }
}
