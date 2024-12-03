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
pub(crate) mod traits;


use hivemqtt_macros::DataSize;

fn main() {}