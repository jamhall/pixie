use core::time;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use std::thread;

use serde::{Deserialize, Serialize};

use pixie::common::{ApplicationError, Colour, Frame};
use pixie::io::{Command, Transport};

#[derive(Serialize, Deserialize, Clone)]
struct Data {
    rows: u32,
    columns: u32,
    padding: u32,
    #[allow(clippy::type_complexity)]
    frames: Vec<Vec<Vec<(u8, u8, u8)>>>,
    delays: Vec<u64>,
}

fn main() {
    let path = std::env::args().nth(1).expect("no file path given");
    let address = std::env::args().nth(2).unwrap_or_else(|| "127.0.0.1:6789".to_string());
    if let Err(error) = process(path, address) {
        println!("Error executing program: {}", error);
    }
}

fn process(path: String, address: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;


    let data: Data = serde_json::from_str(&contents)?;
    let columns = data.columns;

    for (index, frame) in data.frames.into_iter().enumerate() {
        let pixels = frame.iter()
            .flat_map(|row| {
                if row.len() == columns as usize {
                    return row.iter()
                        .map(|(r, g, b)| {
                            Colour::new(*r, *g, *b)
                        })
                        .collect::<Vec<Colour>>();
                }
                panic!("Pixel count mismatch for row");
            })
            .collect::<Vec<Colour>>();

        let frame = Frame::new(data.rows, data.columns, data.padding, pixels);
        let command = Command::Frame(frame);
        send(&address, command)?;

        if !data.delays.is_empty() {
            let millis: &u64 = match data.delays.get(index) {
                Some(millis) => millis,
                None => &200
            };
            let duration = time::Duration::from_millis(*millis);
            thread::sleep(duration);
        }
    }
    thread::sleep(time::Duration::from_millis(1000));
    send(&address, Command::Clear)?;

    Ok(())
}

fn send(address: &str, command: Command) -> Result<(), ApplicationError> {
    let transport = Transport::new();
    let mut stream = BufWriter::new(TcpStream::connect(address)?);
    let encoded = transport.encode(command)?;
    return if let Err(error) = stream.write_all(&encoded) {
        Err(ApplicationError::Transport(format!("Error writing to socket: {}", error)))
    } else {
        Ok(())
    };
}



