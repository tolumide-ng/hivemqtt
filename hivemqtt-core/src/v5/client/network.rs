use std::time::Duration;

use async_channel::Receiver;
use futures::StreamExt;
// use tokio::net::TcpStream;

use crate::v5::commons::{error::MQTTError, packet::Packet};

use super::{handler::{self, AsyncHandler}, state::State, ConnectOptions};
use crate::v5::traits::stream::AsyncStreamExt;


pub struct Network<S, H> {
    state: State,
    stream: Option<S>,
    receiver: Option<Receiver<Packet>>,
    keep_alive: Duration,
    handler: H,
}

impl<S, T, H> Network<S, H> 
    where 
        H: handler::AsyncHandler,
        S: Unpin + Send + Sync + StreamExt<Item = T> + Sized
{
    pub async fn connect(&mut self, mut stream: S, handler: &mut H) -> Result<(), MQTTError> {
        // stream.ready_chunks(capacity)
        let xxx = stream.read().await?;
        Ok(())
    }
}


// mod comp_confirmation {
//     use super::*;

//     struct HST {}
//     impl AsyncHandler for HST {
//         async fn handle(&mut self, packet: Packet) {}
//     }


//     async fn test_impl() {
//         let xxx = HST {};
//         let stream = TcpStream::connect("whatever").await.unwrap();

//         let xx = Network {
//             state: State::new(ConnectOptions::default()),
//             stream: Some(stream),
//             receiver: None,
//             keep_alive: Duration::new(60, 0),
//             handler: xxx,
//         };
//     }
// }
