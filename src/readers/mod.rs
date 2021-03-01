pub use csv_reader::CsvReader;
pub use rss_reader::RssReader;

mod csv_reader;
mod rss_reader;

const MAX_LENGTH: usize = 10;

pub trait Reader {
    fn trim(&mut self);

    fn replace(&mut self, from: &str, to: &str);

    fn display(&mut self) -> String;
}

pub fn determine_reader(url: Option<&str>, input: Option<&str>) -> Box<dyn Reader> {
    if let Some(i) = url {
        Box::new(RssReader::new(i.to_string()))
    } else {
        Box::new(CsvReader::new(input.unwrap().to_string()))
    }
}
