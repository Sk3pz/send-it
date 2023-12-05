use std::io::Read;

use crate::Segment;

/// A reader that reads variable-length encoded data from a stream.
/// Data is read in little-endian unless the big-endian feature is enabled.
/// # Example
/// ```
/// use send_it::reader::VarReader;
///
/// // Create a sample stream, this is the output from the above test_writer test
/// let stream: Vec<u8> = vec![21, 7, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 6, 0, 0, 0, 87, 111, 114, 108, 100, 33];
/// // turn the vector into a slice as Vec does not implement Read
/// let mut fake_stream = stream.as_slice();
///
/// // create a new VarReader
/// let mut reader = crate::reader::VarReader::new(&mut fake_stream);
///
/// let data = reader.read_data().unwrap();
/// assert_eq!(data[0].to_string(), "Hello");
/// assert_eq!(data[1].to_string(), "World");
/// ```
pub struct VarReader<'a, R: Read> {
    reader: &'a mut R,
}

impl<'a, R: Read> VarReader<'a, R> {
    /// Create a new VarReader
    pub fn new(reader: &'a mut R) -> Self {
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

    #[cfg(not(feature = "big-endian"))]
    fn read_u32(&mut self) -> std::io::Result<u32> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    #[cfg(feature = "big-endian")]
    fn read_u32(&mut self) -> std::io::Result<u32> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }

    /// Reads data from the stream.
    /// # Example
    /// ```
    /// use send_it::reader::VarReader;
    ///
    /// let stream: Vec<u8> = vec![21, 7, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 6, 0, 0, 0, 87, 111, 114, 108, 100, 33];
    /// // turn the vector into a slice as Vec does not implement Read
    /// let mut fake_stream = stream.as_slice();
    ///
    /// // create a new VarReader
    /// let mut reader = crate::reader::VarReader::new(&mut fake_stream);
    ///
    /// let data = reader.read_data().unwrap();
    /// assert_eq!(data[0].to_string(), "Hello");
    /// assert_eq!(data[1].to_string(), "World");
    /// ```
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