mod csv_writer;
mod rss_writer;

use crate::readers::Reader;
pub use csv_writer::CsvWriter;
pub use rss_writer::RssWriter;

/// Trait that represents a struct that can perform file writing
pub trait Writer {
    /// Perform a write operation, using the given `File` of the `Writer`
    fn write(&mut self);
}

/// Factory method to determine a writer based on the given arguments
/// Returns a `Writer` to the caller
///
/// # Arguments
/// * `url` - `Option<&str>` of the URL argument in the system
/// * `output` - `&str` of the OUTPUT argument in the system
/// * `reader` - `Box<Reader>` of the given reader used for the input
pub fn determine_writer(
    url: Option<&str>,
    output: &str,
    reader: Box<dyn Reader>,
) -> Box<dyn Writer> {
    if let Some(i) = url {
        Box::new(RssWriter::new(i.to_string(), reader))
    } else {
        Box::new(CsvWriter::new(output.to_string(), reader))
    }
}
