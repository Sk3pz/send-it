mod writer;
mod reader;

#[derive(Debug, Clone)]
pub struct Segment {
    seg: Vec<u8>
}

impl Segment {
    fn new() -> Self {
        Self {
            seg: Vec::new()
        }
    }

    fn append(&mut self, seg: Segment) {
        self.seg.extend(seg.seg);
    }

    pub fn len(&self) -> usize {
        self.seg.len()
    }
}

impl AsRef<[u8]> for Segment {
    fn as_ref(&self) -> &[u8] {
        &self.seg
    }
}

impl From<&[u8]> for Segment {
    fn from(value: &[u8]) -> Self {
        Self {
            seg: value.to_vec()
        }
    }
}

impl From<Vec<u8>> for Segment {
    fn from(value: Vec<u8>) -> Self {
        Self {
            seg: value
        }
    }
}

impl From<String> for Segment {
    fn from(value: String) -> Self {
        Self {
            seg: value.as_bytes().to_vec()
        }
    }
}

impl ToString for Segment {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.seg).to_string()
    }
}

mod tests {

    #[test]
    fn test_writer() {
        let mut sender = crate::writer::VarSender::new();

        // Add some sample data
        sender.add_string("Hello, ");
        sender.add_string("World!");

        // Use any Write implementor as your stream (e.g., File, TcpStream, etc.)
        let mut writer: Vec<u8> = Vec::new();
        sender.send(&mut writer).expect("Failed to send data");

        println!("Compressed data: {:?}", writer);
        
        println!("Compressed data (binary):");
        for byte in &writer {
            print!("{:08b}", byte);
        }
        println!();

        let mut reader = crate::reader::VarReader::new(writer.as_slice());

        match reader.read_data() {
            Ok(original_data) => {
                println!("Original data: {:?}", original_data);
                for (x, seg) in original_data.iter().enumerate() {
                    println!("Segment #{}: {}", x, seg.to_string());
                }
            }
            Err(err) => {
                eprintln!("Failed to read data: {:?}", err);
            }
        }
    }
}