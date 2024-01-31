use std::fmt::Display;

#[cfg(feature="writing")]
#[cfg(not(feature="tokio"))]
pub mod writer;
#[cfg(feature="reading")]
#[cfg(not(feature="tokio"))]
pub mod reader;

#[cfg(feature="writing")]
#[cfg(feature="tokio")]
pub mod async_writer;
#[cfg(feature="reading")]
#[cfg(feature="tokio")]
pub mod async_reader;

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

    pub fn to_readable(segments: Vec<Segment>) -> Vec<String> {
        segments.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    /// Appends a Segment to the end of this Segment.
    pub fn append(&mut self, seg: Segment) {
        self.seg.extend(seg.seg);
    }

    pub(crate) fn len(&self) -> usize {
        self.seg.len()
    }

    pub fn to_raw(&self) -> Vec<u8> {
        self.seg.clone()
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
        // turn the vector into a slice as Vec does not implement Read
        let mut fake_stream = stream.as_slice();

        // create a new VarReader
        let mut reader = crate::reader::VarReader::new(&mut fake_stream);

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

        // turn the vector into a slice as Vec does not implement Read
        let mut fake_stream = stream.as_slice();

        // create a new VarReader
        let mut reader = crate::reader::VarReader::new(&mut fake_stream);

        // read the data from the stream
        let data = reader.read_data().unwrap();
        assert_eq!(data[0].to_string(), "Hello, ");
        assert_eq!(data[1].to_string(), "World!");
    }
}