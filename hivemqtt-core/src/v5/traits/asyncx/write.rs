use std::future::Future;

use crate::v5::commons::error::MQTTError;

pub(crate) trait AsyncWrite<S>: Sized {
    fn writex() -> impl Future<Output = Result<(), MQTTError>>;
}