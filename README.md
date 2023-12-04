# Send-it
[![Crates.io](https://img.shields.io/crates/v/send-it.svg)](https://crates.io/crates/send-it)
[![Docs.rs](https://docs.rs/send-it/badge.svg)](https://docs.rs/send-it)\
A rust crate for sending large segments of data over a stream or network using variable-length encoding.

## Example
```rust
use send_it::writer::VarWriter;
use send_it::reader::VarReader;

// Create a new VarWriter
let mut writer = VarWriter::new();

// Add some sample data
writer.add_string("Hello, ");
writer.add_string("World!");

// Use any Write implementor as your stream (e.g., File, TcpStream, etc.)
let mut stream: Vec<u8> = Vec::new();

// encode the data and send it over the stream
writer.send(&mut stream).expect("Failed to send data");

// create a new VarReader to read from the stream we wrote to
let mut reader = VarReader::new(stream.as_slice());

// read the data from the stream
let data = reader.read_data().unwrap();
assert_eq!(data[0].to_string(), "Hello, ");
assert_eq!(data[1].to_string(), "World!");
```

## Features
#### writing (enabled by default)
Adds the VarWriter struct, which is used to write data to a stream using variable-length encodingq
#### reading (enabled by default)
Adds the VarReader struct, which is used to read data from a stream using variable-length encoding
#### big-endian (disabled by default)
Changes the encoding to use big-endian instead of little-endian

## Usage
### VarWriter
A struct used to write data to a stream using variable-length encoding\
feature: 'writing' (enabled by default)
```rust
use send_it::writer::VarWriter;

// Create a new VarWriter
let mut writer = VarWriter::new();

// Add some sample data
writer.add_string("Hello, ");
writer.add_string("World!");

// Use any Write implementor as your stream (e.g., File, TcpStream, etc.)
let mut stream: Vec<u8> = Vec::new();

// encode the data and send it over the stream
writer.send(&mut stream).expect("Failed to send data");
```

### VarReader
A struct used to read data from a stream using variable-length encoding\
feature: 'reading' (enabled by default)
```rust
use send_it::reader::VarReader;

// Create a sample stream, this is the output from the above VarWriter example
let stream: Vec<u8> = vec![21, 7, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 6, 0, 0, 0, 87, 111, 114, 108, 100, 33];

// create a new VarReader
let mut reader = VarReader::new(stream.as_slice());

// read the data from the stream
let data = reader.read_data().unwrap();
assert_eq!(data[0].to_string(), "Hello, ");
assert_eq!(data[1].to_string(), "World!");
```

### Segment
A struct used to represent a segment of data
```rust
use send_it::Segment;
let mut segment = Segment::new();
segment.append(Segment::from("Hello, "));
segment.append(Segment::from("World!"));
assert_eq!(segment.to_string(), "Hello, World!");
```
