use std::io::Read;

use crate::Segment;

pub struct VarReader<R: Read> {
    reader: R,
}

impl<R: Read> VarReader<R> {
    pub fn new(reader: R) -> Self {
        VarReader { reader }
    }

    fn read_varint(&mut self) -> std::io::Result<usize> {
        let mut value = 0usize;
        let mut shift = 0;
        loop {
            let mut buf = [0; 1];
            self.reader.read_exact(&mut buf)?;
            let byte = buf[0];
            value |= ((byte & 0x7F) as usize) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(value)
    }

    fn read_u32(&mut self) -> std::io::Result<u32> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_data(&mut self) -> std::io::Result<Vec<Segment>> {
        let total_size = self.read_varint()?;
        let mut data = Vec::new();

        while data.iter().map(|segment: &Segment| segment.len() + 4).sum::<usize>() < total_size {
            let segment_size = self.read_u32()? as usize;
            let mut segment_data = vec![0u8; segment_size];
            self.reader.read_exact(&mut segment_data)?;
            data.push(Segment::from(segment_data));
        }

        Ok(data)
    }
}