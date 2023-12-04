use std::fmt::Display;

#[cfg(feature="writing")]
pub mod writer;
#[cfg(feature="reading")]
pub mod reader;

/// A segment of data used by VarReader and VarWriter to send and receive data over a stream.
/// # Examples
/// ```
/// use send_it::Segment;
///
/// let mut segment = Segment::new();
/// segment.append(Segment::from("Hello, "));
/// segment.append(Segment::from("World!"));
///
/// assert_eq!(segment.to_string(), "Hello, World!");
/// ```
#[derive(Debug, Clone)]
pub struct Segment {
    seg: Vec<u8>
}

impl Segment {
    /// Creates a new Segment.
    pub fn new() -> Self {
        Self {
            seg: Vec::new()
        }
    }

    /// Appends a Segment to the end of this Segment.
    pub fn append(&mut self, seg: Segment) {
        self.seg.extend(seg.seg);
    }

    pub(crate) fn len(&self) -> usize {
        self.seg.len()
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self::new()
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

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.seg))
    }
}

mod tests {

    #[test]
    fn test_writer() {
        // Create a new VarWriter
        let mut writer = crate::writer::VarWriter::new();

        // Add some sample data
        writer.add_string("Hello, ");
        writer.add_string("World!");

        // Use any Write implementor as your stream (i.e. TcpStream)
        let mut stream: Vec<u8> = Vec::new();

        // encode the data and send it over the stream
        writer.send(&mut stream).expect("Failed to send data");
    }

    #[test]
    fn test_reader() {
        // Create a sample stream, this is the output from the above test_writer test
        let stream: Vec<u8> = vec![21, 7, 0, 0, 0, 72, 101, 108, 108, 111, 44, 32, 6, 0, 0, 0, 87, 111, 114, 108, 100, 33];

        // create a new VarReader
        let mut reader = crate::reader::VarReader::new(stream.as_slice());

        // read the data from the stream
        let data = reader.read_data().unwrap();
        assert_eq!(data[0].to_string(), "Hello, ");
        assert_eq!(data[1].to_string(), "World!");
    }

    #[test]
    fn both_test() {
        // Create a new VarWriter
        let mut writer = crate::writer::VarWriter::new();

        // Add some sample data
        writer.add_string("Hello, ");
        writer.add_string("World!");

        // Use any Write implementor as your stream (i.e. TcpStream)
        let mut stream: Vec<u8> = Vec::new();

        // encode the data and send it over the stream
        writer.send(&mut stream).expect("Failed to send data");

        // create a new VarReader to read from the stream we wrote to
        let mut reader = crate::reader::VarReader::new(stream.as_slice());

        // read the data from the stream
        let data = reader.read_data().unwrap();
        assert_eq!(data[0].to_string(), "Hello, ");
        assert_eq!(data[1].to_string(), "World!");
    }
}