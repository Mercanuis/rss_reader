use std::fs::File;
use std::io::Write;

use crate::readers::Reader;

use super::Writer;

///Represents a CSV feed writer, based on an `CsvReader`
pub struct CsvWriter {
    ///The name or path of the file to write to
    file_name: String,
    ///The `Reader`, in a `Box` to allow the use of dynamic access during compile time
    reader: Box<dyn Reader>,
}

impl CsvWriter {
    pub fn new(file_name: String, reader: Box<dyn Reader>) -> Self {
        Self { file_name, reader }
    }
}

impl Writer for CsvWriter {
    fn write(&mut self) {
        let mut file = File::create(self.file_name.clone())
            .unwrap_or_else(|_| File::create("./example_files/default.txt").unwrap());
        let res = format!("{:?}", &self.reader.display());
        file.write_all(res.as_ref()).unwrap();
    }
}
