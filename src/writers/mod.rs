mod csv_writer;
mod rss_writer;

use crate::readers::Reader;
pub use csv_writer::CsvWriter;
pub use rss_writer::RssWriter;

pub trait Writer {
    fn write(&mut self);
}

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
