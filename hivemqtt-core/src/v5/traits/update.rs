use crate::v5::commons::{error::MQTTError, property::new_approach::Property};

// use super::{bufferio::BufferIO, streamio::StreamIO};

pub(crate) fn try_update<T>(
    field: &mut Option<T>,
    value: Option<T>,
) -> impl Fn(Property) -> Result<(), MQTTError> {
    let is_duplicate = field.is_some();
    *field = value;

    move |ppt| {
        if is_duplicate {
            return Err(MQTTError::DuplicateProperty(ppt.to_string()));
        }
        return Ok(());
    }
}
