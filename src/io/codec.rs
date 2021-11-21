use bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use crate::common::ApplicationError;
use crate::io::transport::{Command, CommandResponse, Transport};

pub struct CommandCodec {
    transport: Transport,
}

impl CommandCodec {
    pub fn new() -> Self {
        Self {
            transport: Transport::new()
        }
    }
}


impl Encoder<CommandResponse> for CommandCodec {
    type Error = ApplicationError;

    fn encode(&mut self, response: CommandResponse, destination: &mut BytesMut) -> Result<(), Self::Error> {
        match response {
            CommandResponse::Accepted => {
                destination.put_u8(1);
            }
            CommandResponse::Refused(message) => {
                let bytes = message.as_bytes();
                let length = bytes.len() as u32;
                destination.put_u8(0);
                destination.put_u32(length);
                destination.put_slice(bytes);
            }
        }
        Ok(())
    }
}


impl Decoder for CommandCodec {
    type Item = Command;
    type Error = ApplicationError;

    // decode a command when it is sent from a client
    fn decode(&mut self, source: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if !source.is_empty() {
            return if let Ok(command) = self.transport.decode(source) {
                Ok(Some(command))
            } else {
                Err(ApplicationError::InvalidCommand("Command not known".to_string()))
            };
        }
        Ok(None)
    }
}

impl Default for CommandCodec {
    fn default() -> Self {
        Self::new()
    }
}