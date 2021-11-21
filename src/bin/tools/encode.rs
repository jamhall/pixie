use core::time;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::net::TcpStream;
use std::thread;

use serde::{Deserialize, Serialize};

use pixie::common::{ApplicationError, Colour, Coordinate, Frame, Pixel};
use pixie::io::{Command, Transport};

#[derive(Serialize, Deserialize)]
struct Data {
    #[allow(clippy::type_complexity)]
    frames: Vec<Vec<Vec<(f64, f64, f64, f64)>>>,
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

    for (index, frame) in data.frames.into_iter().enumerate() {
        let pixels = frame.into_iter().enumerate()
            .flat_map(|(y, row)| {
                row.into_iter().enumerate().map(|(x, (r, g, b, _))| {
                    let coordinate = Coordinate::new(x as u32, y as u32);
                    let colour = Colour::new(
                        (255.0 * r).round() as u8,
                        (255.0 * g).round() as u8,
                        (255.0 * b).round() as u8,
                    );
                    Pixel::new(coordinate, colour)
                }).collect::<Vec<Pixel>>()
            })
            .collect::<Vec<Pixel>>();

        let frame = Frame::new(8, 8, 10, pixels);
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

