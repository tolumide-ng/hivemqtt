pub(crate) trait StreamIO: Stream {
    /// Encodes a non-negative Integer into the Variable Byte Integer encoding
    async fn encode(&self) -> Result<Vec<u8>, MQTTError> {
        let mut len = self.length();
        let mut result = vec![];

        // 268_435_455
        if len > 0xFFFFFFF {
            return Err(MQTTError::PayloadTooLong);
        }

        for _ in 0..4 {
            let mut byte = len % 128;
            len /= 128;

            if len > 0 {
                byte |= 128;
            }

            // (byte as u8).write(stream).await?; // writes the encoded byte into the buffer
            result.push(byte as u8);
            if len == 0 {
                break;
            }
        }

        return Ok(result);
    }

    /// Decodes a Variable byte Integer
    async fn decode(stream: &mut R) -> Result<(usize, usize), MQTTError> {
        let mut result = 0;

        for i in 0..4 {
            let byte = u8::read(stream).await?;

            result += ((byte as usize) & 0x7F) << (7 * i);

            if (byte & 0x80) == 0 {
                return Ok((result, i + 1));
            }
        }

        return Err(MQTTError::MalformedPacket);
    }
}
