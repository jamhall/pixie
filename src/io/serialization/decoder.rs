use std::convert::TryFrom;

use bytes::{Buf, BytesMut};

use crate::common::{ApplicationError, Colour, Frame};
use crate::io::{Command, CommandType};

pub struct Decoder {}

impl Decoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode(&self, bytes: &mut BytesMut) -> Result<Command, ApplicationError> {
        // parse the command id, otherwise throw an error if unknown
        let id = bytes.get_u8();
        if let Ok(command_type) = CommandType::try_from(id) {
            return match command_type {
                CommandType::Frame => self.decode_frame(bytes),
                CommandType::Clear => Ok(self.decode_clear())
            };
        }
        Err(ApplicationError::InvalidCommand(format!("Unknown command id: {}", id)))
    }

    pub fn decode_frame(&self, reader: &mut BytesMut) -> Result<Command, ApplicationError> {
        let rows = reader.get_u32_le();
        let columns = reader.get_u32_le();
        let padding = reader.get_u32_le();
        let range = 0..rows * columns;
        // a nice bit of functional rust...
        let pixels: Vec<Colour> = range
            .into_iter()
            .map(|_| {
                let (r, g, b) = self.decode_colour(reader);
                Colour::new(r, g, b)
            })
            .collect::<Vec<Colour>>();
        let frame = Frame::new(rows, columns, padding, pixels);
        Ok(Command::Frame(frame))
    }

    fn decode_colour(&self, reader: &mut BytesMut) -> (u8, u8, u8) {
        let r = reader.get_u8();
        let g = reader.get_u8();
        let b = reader.get_u8();
        (r, g, b)
    }

    fn decode_clear(&self) -> Command {
        Command::Clear
    }
}