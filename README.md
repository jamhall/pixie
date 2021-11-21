# pixie

A simple project for presenting a display of pixels written in rust. It uses the sdl2 library for rendering and tokio
for handling server events 🦀


> This is my first project to learn Rust so there is likely code that is not very idiomatic and may not conform to best practices (but hey, i'm learning)

### Description

The objective of this (personal) project is to replicate the functionality of [LaMetric](https://lametric.com/en-US).

I wanted the display to be independent of the controller sending the updates, i.e. it only handles the rendering of
frames. This is because in the future, I may want to plug-in a LED pixel display as the display driver.

The application will start a TCP server on a port of your choice (defaults to 6789) and then waits to receive a command.

The following commands are supported:

| Command ID | Name    | Description                    |           
| ---------- | ------- | ------------------------------ |
| 1          | Frame   | Renders a frame to the display |
| 2          | Clear   | Clears the display             |

The application uses a custom binary format for decoding the commands received (yes, yes, I could have used protobuf or
something else but where's the fun in that 😀)

I also didn't use the popular crates such as [anyhow](https://lib.rs/crates/anyhow)
, [thiserror](https://lib.rs/crates/thiserror) or  [failure](https://lib.rs/crates/failure) as I wanted to get a better
understanding of how error handling works in rust.

![Screenshot](assets/screenshot.png?raw=true)

### Getting started

#### Starting the display

Run the following command to start the display:

```bash
RUST_LOG=DEBUG cargo run --bin server -- -w 240 -h 240
```

```bash
pixie 0.1.0

Jamie Hall <hello@jamiehall.eu>

Displays a grid of pixels

USAGE:
    pixie [OPTIONS]

OPTIONS:
USAGE:
    pixie [OPTIONS]

OPTIONS:
    -h <height>        display height [default: 240]
        --help         Print help information
    -s <server>        server address [default: 127.0.0.1:6789]
    -V, --version      Print version information
    -w <width>         display width [default: 960]


```

#### Sending a test frame

You can run the following command to send a test frame to the display:

```bash
cargo run --bin encode -- assets/rainbow.json
```

You should see a nice rainbow animation (like the screenshot above)

The control server will be sending binary, but for the sake of simplicity this example accepts a JSON file and encodes
it into the binary format required to display a frame.

## Binary protocol

### Frame binary structure

A frame is a vector of rows and columns. Let's assume we have a grid of _3x6_ (3 rows and 6 columns) pixels. Here's an
example representation of a grid of pixels:

```
    [rgb(255, 0, 0), rgb(248, 255, 41), rgb(255, 0, 0), rgb(255, 0, 0), rgb(255, 0, 0), rgb(255, 0, 0)],
    [rgb(0, 255, 0), rgb(248, 255, 41), rgb(0, 255, 0), rgb(0, 255, 0), rgb(0, 255, 0), rgb(0, 255, 0)],
    [rgb(0, 0, 255), rgb(248, 255, 41), rgb(0, 0, 255), rgb(0, 0, 255), rgb(0, 0, 255), rgb(0, 0, 255)]
```

This would render to the display:

```
    Row 1: red (x: 0, y: 0), yellow (x: 1, y: 0), red (x: 2, y: 0), red (x: 3, y: 0), red (x: 4, y: 0), red (x: 5, y: 0) 
    Row 2: green  (x: 0, y: 1), yellow (x: 1, y: 1), green (x: 2, y: 1), green (x: 3, y: 1), green (x: 4, y: 1), green (x: 5, y: 1)
    Row 3: blue (x: 0, y: 2), yellow (x: 1, y: 2), blue (x: 2, y: 2), blue (x: 3, y: 2), blue (x: 4, y: 2), blue (x: 5, y: 2)
```

For a display of 8x32 pixels the number of pixels to send will be 256 multiplied by 3 bytes (r, g, b colour values)

| Bytes | Definition | Type                     | Description                  |
| ----- | ---------- | ------------------------ | ---------------------------- |
| 1     | command    | u8 (1 byte)              | Command                      |
| 5     | rows       | u32 (4 bytes)  | Numbers of rows (i.e. 8)     |
| 9     | columns    | u32  (4 bytes) | Numbers of columns (i.e. 32) |
| 13     | padding    | u32  (4 bytes) | Padding between the pixels (i.e. 10) |
| 14    | r          | u8 (1 byte)              | Red (0 - 255)                |
| 15    | g          | u8 (1 byte)              | Green (0 - 255)              |
| 16    | b          | u8 (1 byte)              | Blue (0 - 255)               |
