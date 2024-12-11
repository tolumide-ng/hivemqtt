// If there are no properties, the Property Length must be indicated by including a Property length of zero(0)

macro_rules! packet_props {
    ($($property:ident),*) => {
        {
            0u64 $(| (1u64 << $property as u64))*
        }
    };
}



pub mod constants;
pub mod packet;
pub mod commons;
pub mod traits;


use bytes::{Buf, Bytes};
use commons::error::MQTTError;
use hivemqtt_macros::Length;
use traits::read::Read;

fn main() {}
