use std::marker::PhantomData;

use futures::{AsyncReadExt, AsyncWriteExt};

use crate::v5::client::{client::MqttClient, handler::AsyncHandler, ConnectOptions};

#[derive(Debug)]
pub struct Network<H, S> {
    stream: PhantomData<S>,
    handler: PhantomData<H>,
    options: ConnectOptions,
    client: MqttClient,
}

impl<H, S> Network<H, S>
where
    H: AsyncHandler,
    S: AsyncReadExt + AsyncWriteExt + Unpin,
{
    fn new(options: ConnectOptions) -> Self {
        let client = MqttClient::new();
        Self {
            stream: PhantomData,
            handler: PhantomData,
            options,
            client,
        }
    }
}
