// mod comp_confirmation {
//     use tokio::net::TcpStream;
//     use tokio_util::compat::TokioAsyncReadCompatExt;

//     use super::*;

//     struct HST {}
//     impl AsyncHandler for HST {
//         async fn handle(&mut self, packet: Packet) {}
//     }

//     async fn test_impl() {
//         let xxx = HST {};
//         // let stream = tokio::net::TcpStream::connect("whatever").await.unwrap();
//         let stream = TcpStream::connect("example.com:80").await.unwrap();
//         // let stream = tokio::net::TcpStream::connect("hostname").await.unwrap();
//         // let stream = tokio::io::BufStream::new(stream);

//         let stream = stream.compat();

//         let mut xx = Network {
//             state: State::new(ConnectOptions::default()),
//             stream: Some(stream),
//             receiver: None,
//             keep_alive: Duration::new(60, 0),
//             handler: xxx,
//         };

//         let stream = TcpStream::connect("example.com:80").await.unwrap();
//         let stream = stream.compat();
//         let abc = xx.connect(stream);
//     }
// }

use dotenvy::dotenv;
use hivemqtt_core::v5::{
    client::{handler::AsyncHandler, network::asyncx::Network, ConnectOptions},
    commons::packet::Packet,
};
use std::env;
use tokio_util::compat::TokioAsyncReadCompatExt;

pub(crate) struct Handler;

impl AsyncHandler for Handler {
    async fn handle(&mut self, packet: Packet) -> () {
        ()
    }
}

#[tokio::main]
fn main() {
    dotenv().ok();
    let hostname = env::var("HOSTNAME").expect("HOSTNAME required");
    let username = Some(env::var("USERNAME").expect("USERNAME required"));
    let password = Some(env::var("PASSWORD").expect("PASSWORD required"));

    let mut handler = Handler;

    let stream = tokio::net::TcpStream::connect(hostname).await.unwrap();
    let stream = tokio::io::BufStream::new(stream);
    let stream = stream.compat();

    let options = ConnectOptions {
        username,
        password,
        ..Default::default()
    };

    let network = Network::new(options, stream).await;
    let (mut result, client) = network.inspect(|x| println!("x {:?}", x)).unwrap();

    tokio::spawn(async move {
        println!("now running!!!!!!!!");
        result.run(&mut handler).await;
    });

    println!("ABOUT TO DISCONNECT!!!***");
    client.disconnect().await;
    println!("disconnected!!!");
}
