use std::time::Duration;

use async_channel::Receiver;

use crate::v5::commons::{error::MQTTError, packet::Packet};

use super::{handler, state::State};

pub struct Network<S, H> {
    state: State,
    stream: Option<S>,
    receiver: Receiver<Packet>,
    keep_alive: Duration,
    handler: H,
}

// impl<S, H> Network<S, H> where H: handler::AsyncHandler, S: AsyncReadExt + AsyncWriteExt + Unpin + Send + Sized {
//     pub async fn connect(&mut self, mut stream: S, handler: &mut H) -> Result<(), MQTTError> {
//         // let connack = stream.conn
//         Ok(())
//     }
// }