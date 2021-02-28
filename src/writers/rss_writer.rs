use std::fs::File;
use std::io::Write;

use crate::readers::Reader;

use super::Writer;

pub struct RssWriter {
    file_name: String,
    reader: Box<dyn Reader>,
}

impl RssWriter {
    pub fn new(file_name: String, reader: Box<dyn Reader>) -> Self {
        Self { file_name, reader }
    }
}

impl Writer for RssWriter {
    fn write(&mut self) {
        let mut file = File::create(self.file_name.clone()).unwrap();
        let res = format!("{:?}", &self.reader.display());
        file.write_all(res.as_ref()).unwrap();
    }
}
