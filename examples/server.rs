use send_it::Segment;

fn main() {
    // create a TcpListener
    let listener = std::net::TcpListener::bind("0.0.0.0:3333").expect("Failed to start tcp listener!");
    println!("Listening on :3333");

    // listen for incoming connections
    for stream in listener.incoming() {
        let mut stream = stream.expect("Invalid stream!");
        
        // handle a new connection
        std::thread::spawn(move || {
            // create a reader for the TcpStream
            let mut reader = send_it::reader::VarReader::new(&mut stream);
            // loop while there is incoming data
            while let Ok(data) = reader.read_data() {
                // convert our segments to strings in the vector
                let readable_data = Segment::to_readable(data);
                
                // print out the vector of strings
                println!("Segments from client: [{}]", readable_data.join(", "));
            }
            println!("Connection closed!");
        });
    }
}