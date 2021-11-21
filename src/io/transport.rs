use bytes::BytesMut;
use num_enum::TryFromPrimitive;

use crate::common::{ApplicationError, Frame};
use crate::io::serialization::{Decoder, Encoder};

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum CommandType {
    Frame = 1,
    Clear = 2,
}

#[derive(Clone, Debug)]
pub enum Command {
    Frame(Frame),
    Clear,
    None,
}

pub enum CommandResponse {
    Accepted,
    Refused(String),
}

pub struct Transport {
    encoder: Encoder,
    decoder: Decoder,
}


#[allow(dead_code)]
impl Transport {
    pub fn new() -> Self {
        Self {
            encoder: Encoder::new(),
            decoder: Decoder::new(),
        }
    }

    pub fn decode(&self, bytes: &mut BytesMut) -> Result<Command, ApplicationError> {
        self.decoder.decode(bytes)
    }

    pub fn encode(&self, command: Command) -> Result<Vec<u8>, ApplicationError> {
        self.encoder.encode(command)
    }
}

impl Default for Transport {
    fn default() -> Self {
        Self::new()
    }
}
