use std::io::Write;
use crate::Segment;


/// A writer for sending several segments over a stream using variable length encoding
/// Data is written in little-endian if the feature "big-endian" is not enabled
/// # Example
/// ```
/// use send_it::writer::VarWriter;
///
/// let mut sender = VarWriter::new();
///
/// sender.add_string("Hello");
/// sender.add_string("World");
///
/// let mut buffer = Vec::new();
/// sender.send(&mut buffer).unwrap();
/// ```
pub struct VarWriter {
    data: Vec<Segment>,
}

impl VarWriter {
    /// Create a new VarWriter
    pub fn new() -> VarWriter {
        VarWriter {
            data: Vec::new(),
        }
    }

    /// Add a segment to the writer
    pub fn add(&mut self, segment: Segment) {
        self.data.push(segment);
    }

    /// Add a string to the writer
    /// # Example
    /// ```
    /// use send_it::writer::VarWriter;
    ///
    /// let mut sender = VarWriter::new();
    ///
    /// sender.add_string("Hello");
    /// ```
    pub fn add_string<S: Into<String>>(&mut self, string: S) {
        self.add(Segment::from(string.into()))
    }

    /// Add raw data to the writer
    /// # Example
    /// ```
    /// use send_it::writer::VarWriter;
    ///
    /// let mut sender = VarWriter::new();
    ///
    /// sender.add_raw(&[0x48, 0x65, 0x6C, 0x6C, 0x6F]);
    /// ```
    pub fn add_raw(&mut self, raw: &[u8]) {
        self.data.push(Segment::from(raw));
    }

    /// Encodes the data and sends it over the stream.
    /// * The data is cleared after sending.
    /// # Example
    /// ```
    /// use send_it::writer::VarWriter;
    ///
    /// let mut sender = VarWriter::new();
    ///
    /// sender.add_string("Hello");
    /// sender.add_string("World");
    ///
    /// let mut buffer = Vec::new();
    /// sender.send(&mut buffer).unwrap();
    /// ```
    pub fn send<W: Write>(&mut self, stream: &mut W) -> std::io::Result<()> {
        self.send_without_clearing(stream)?;

        // Clear the internal data after sending
        self.clear();

        Ok(())
    }

    /// Encodes the data and sends it over the stream.
    /// * The data is not cleared after sending.
    /// # Example
    /// ```
    /// use send_it::writer::VarWriter;
    ///
    /// let mut sender = VarWriter::new();
    ///
    /// sender.add_string("Hello");
    /// sender.add_string("World");
    ///
    /// let mut buffer = Vec::new();
    /// sender.send_without_clearing(&mut buffer).unwrap();
    /// ```
    pub fn send_without_clearing<W: Write>(&mut self, stream: &mut W) -> std::io::Result<()> {
        let total_size: usize = self.data.iter().map(|segment| segment.len() + 4).sum();

        // Write the total size as varint
        self.write_varint(stream, total_size)?;

        // Write each segment's size and the segment itself
        for segment in &self.data {
            self.write_u32(stream, segment.len() as u32)?;
            // write the segment
            stream.write_all(segment.as_ref())?;
        }

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

    #[cfg(not(feature = "big-endian"))]
    fn write_u32<W: Write>(&self, writer: &mut W, value: u32) -> std::io::Result<()> {
        // writes little-endian
        writer.write_all(&[
            (value & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 24) & 0xFF) as u8,
        ])?;
        Ok(())
    }

    #[cfg(feature = "big-endian")]
    fn write_u32<W: Write>(&self, writer: &mut W, value: u32) -> std::io::Result<()> {
        // writes big-endian
        writer.write_all(&[
            ((value >> 24) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        ])?;
        Ok(())
    }

    /// Removes all segments from the writer
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Write for VarWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let vec = buf.to_vec();
        let size = vec.len();
        self.data.push(Segment::from(vec));
        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // nothing to do
        Ok(())
    }
}

impl Default for VarWriter {
    fn default() -> Self {
        Self::new()
    }
}