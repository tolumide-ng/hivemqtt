macro_rules! packet_props {
    ($($property:ident),*) => {
        {
            0u64 $(| (1u64 << $property as u64))*
        }
    };
}


pub mod variable_header;
pub mod packet_type;
pub mod property;
pub mod reason_code;
pub mod qos;
pub(crate) mod variable_byte_integer;
pub(crate) mod error;
pub(crate) mod version;
pub(crate) mod fixed_header;
pub(crate) mod will_properties;