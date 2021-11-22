use byteorder::{LittleEndian, WriteBytesExt};

use crate::common::ApplicationError;
use crate::io::Command;

pub struct Encoder {}

impl Encoder {
    pub fn new() -> Self {
        Self {}
    }
    pub fn encode(&self, command: Command) -> Result<Vec<u8>, ApplicationError> {
        let mut writer = vec![];
        return match command {
            Command::Frame(frame) => {
                // write command id
                writer.write_u8(1)?;
                // rows
                writer.write_u32::<LittleEndian>(frame.rows())?;
                // writer
                writer.write_u32::<LittleEndian>(frame.columns())?;
                // padding
                writer.write_u32::<LittleEndian>(frame.padding())?;

                // write the colours
                for pixel in frame.pixels() {
                    let (_, colour) = pixel.split();
                    let (r, g, b) = colour.tuple();
                    writer.write_u8(r)?;
                    writer.write_u8(g)?;
                    writer.write_u8(b)?;
                }
                Ok(writer)
            }
            Command::Clear => {
                writer.write_u8(2)?;
                Ok(writer)
            }
            Command::None => {
                Ok(writer)
            }
        };
    }
}