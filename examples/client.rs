

fn main() {
    // create a connection to our example server in server.rs
    let mut connection = std::net::TcpStream::connect("localhost:3333").expect("Failed to connect to server!");
    
    // create a writer
    let mut writer = send_it::writer::VarWriter::default();
    
    // add two segments: "Hello, " and "world!"
    writer.add_string("Hello, ");
    writer.add_string("world!");
    
    // send our two segments to the TcpStream and exit
    writer.send(&mut connection).expect("Failed to send data!");
    println!("sent data!");
}