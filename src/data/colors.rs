use crate::{
    data::{Color, OpenRGBWritable},
    protocol::OpenRGBWritableStream,
    OpenRGBError,
};
use async_trait::async_trait;
use std::mem::size_of;

/// A list of colors.
#[derive(Debug)]
pub struct Colors {
    data: Vec<Color>,
}

impl From<Vec<Color>> for Colors {
    fn from(data: Vec<Color>) -> Self {
        Self { data }
    }
}

#[async_trait]
impl OpenRGBWritable for Colors {
    fn size(&self, _protocol: u32) -> usize {
        size_of::<u32>() + size_of::<u16>() + self.data.len() * 4 * size_of::<u8>()
    }

    async fn write(
        self,
        stream: &mut impl OpenRGBWritableStream,
        protocol: u32,
    ) -> Result<(), OpenRGBError> {
        // Write the payload size
        stream
            .write_value(
                u32::try_from(self.size(protocol)).map_err(|e| {
                    OpenRGBError::ProtocolError(format!("Too many colors to encode: {}", e))
                })?,
                protocol,
            )
            .await?;

        // Write the number of colors
        stream
            .write_value(
                u16::try_from(self.data.len()).map_err(|e| {
                    OpenRGBError::ProtocolError(format!("Too many colors to encode: {}", e))
                })?,
                protocol,
            )
            .await?;

        // Write the colors
        for color in self.data {
            stream.write_value(color, protocol).await?;
        }

        Ok(())
    }
}
