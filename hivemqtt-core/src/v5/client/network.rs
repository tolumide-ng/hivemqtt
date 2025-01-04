use std::time::Duration;

use async_channel::Receiver;

use crate::v5::commons::packet::Packet;

use super::state::State;

// #[cfg(feature = "smol")]
// type Stream<T: smol::io::AsyncRead + smol::io::AsyncWrite + Unpin + Sized> = T;
// #[cfg(feature = "tokio")]
// type Stream<T: smol::io::AsyncRead + smol::io::AsyncWrite + Unpin + Sized> = T;

pub(crate) struct Network<S, H> {
    state: State,
    stream: Option<S>,
    receiver: Receiver<Packet>,
    keep_alive: Duration,
    handler: H,
}

impl<S, H> Network<S, H> {}