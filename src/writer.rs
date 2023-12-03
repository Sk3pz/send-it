use std::io::Write;

pub struct DataWriteBuffer {
    data: Vec<Vec<u8>>,
}

impl DataWriteBuffer {
    pub fn new() -> DataWriteBuffer {
        DataWriteBuffer {
            data: Vec::new(),
        }
    }
    
    pub fn add_segment(&mut self, segment: Vec<u8>) {
        self.data.push(segment);
    }
    
    pub fn add(&mut self, raw: &[u8]) {
        self.data.push(raw.to_vec());
    }

    // send data over a given stream using the varint format
    pub fn send<W: Write>(&mut self, stream: &mut W) -> std::io::Result<()> {
        let total_size: usize = self.data.iter().map(|segment| segment.len() + 4).sum();
        
        // Write the total size as varint
        self.write_varint(stream, total_size)?;
        
        // Write each segment's size and the segment itself
        for segment in &self.data {
            self.write_u32(stream, segment.len() as u32)?;
            stream.write_all(segment)?;
        }

        // Clear the internal data after sending
        self.clear();

        Ok(())
    }
    
    fn write_varint<W: Write>(&self, writer: &mut W, mut value: usize) -> std::io::Result<()> {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            writer.write_all(&[byte])?;
            if value == 0 {
                break;
            }
        }
        Ok(())
    }

    fn write_u32<W: Write>(&self, writer: &mut W, value: u32) -> std::io::Result<()> {
        writer.write_all(&[
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
            ])?;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl std::io::Write for DataWriteBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vec = buf.to_vec();
        let size = vec.len();
        self.data.push(vec);
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // nothing to do
        Ok(())
    }
}