pub use csv_reader::CsvReader;
pub use rss_reader::RssReader;

mod csv_reader;
mod rss_reader;

const MAX_LENGTH: usize = 10;

/// A Trait that describes a way to read input
/// This trait is like a Java Interface, which allows for extension for new concrete classes to implement
pub trait Reader {
    /// Performs a 'trim' operation. Trimming will take the Reader's input and perform a truncation based on
    /// fields specific to the Reader
    fn trim(&mut self);

    /// Performs a 'replace' operation. Replace is a simple 'in-line' replacement of a pattern A with pattern B
    ///
    /// # Arguments
    /// * `from` - The Pattern to search for
    /// * `to` - The Pattern to replace `from`
    fn replace(&mut self, from: &str, to: &str);

    /// Displays the output of the Reader
    /// Returns a `String` of the Reader's information
    fn display(&mut self) -> String;
}

/// Factory method to determine a reader based on the given arguments
/// Returns a `Reader` to the caller
///
/// # Arguments
/// * `url` - `Option<&str>` of the URL argument in the system
/// * `input` - `Option<&str>` of the INPUT argument in the system
pub fn determine_reader(url: Option<&str>, input: Option<&str>) -> Box<dyn Reader> {
    if let Some(i) = url {
        Box::new(RssReader::new(i.to_string()))
    } else {
        Box::new(CsvReader::new(input.unwrap().to_string()))
    }
}
