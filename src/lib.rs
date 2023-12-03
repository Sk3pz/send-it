mod writer;
mod reader;

mod tests {
    #[test]
    fn test_writer() {
        let mut wrapper = crate::writer::DataWriteBuffer::new();

        // Add some sample data
        wrapper.add(b"Hello, ");
        wrapper.add(b"World!");

        // Use any Write implementor as your stream (e.g., File, TcpStream, etc.)
        let mut writer: Vec<u8> = Vec::new();
        wrapper.send(&mut writer).expect("Failed to send data");

        println!("Compressed data: {:?}", writer);
        
        println!("Compressed data (binary):");
        for byte in &writer {
            print!("{:08b}", byte);
        }
        println!();
    }
}