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
                writer.write_u8(1).unwrap();
                // rows
                writer.write_u32::<LittleEndian>(frame.rows()).unwrap();
                // writer
                writer.write_u32::<LittleEndian>(frame.columns()).unwrap();
                // padding
                writer.write_u32::<LittleEndian>(frame.padding()).unwrap();

                // write the colours
                for pixel in frame.pixels() {
                    let (_, colour) = pixel.split();
                    let (r, g, b) = colour.tuple();
                    writer.write_u8(r).unwrap();
                    writer.write_u8(g).unwrap();
                    writer.write_u8(b).unwrap();
                }
                Ok(writer)
            }
            Command::Clear => {
                writer.write_u8(2).unwrap();
                Ok(writer)
            }
            Command::None => {
                Ok(writer)
            }
        };
    }
}