use csv::StringRecord;

use crate::readers::Reader;

use super::MAX_LENGTH;

///Represents a CSV file reader
pub struct CsvReader {
    ///Describes the rows and values taken from the CSV file
    values: Vec<Vec<String>>,
}

impl CsvReader {
    /// Create a new `CsvReader`
    ///
    /// # Arguments
    /// * `file_name` - `String` of the CSV file
    pub fn new(file_name: String) -> Self {
        let mut rdr = csv::Reader::from_path(file_name.clone()).unwrap();
        let mut records = Vec::new();
        for res in rdr.records() {
            let record = res.unwrap_or_else(|_| StringRecord::new());
            records.push(record);
        }
        let mut values: Vec<Vec<String>> = vec![];

        for r in records {
            let mut new_values = vec![];
            for f in r.iter() {
                new_values.push(f.to_string());
            }
            values.push(new_values);
        }

        Self { values }
    }
}

impl Reader for CsvReader {
    fn trim(&mut self) {
        for row in &mut self.values {
            for d in row {
                if d.len().ge(&MAX_LENGTH) {
                    d.truncate(MAX_LENGTH);
                }
            }
        }
    }

    fn replace(&mut self, from: &str, to: &str) {
        for row in &mut self.values {
            for d in row {
                *d = (*d.replace(from, to)).parse().unwrap();
            }
        }
    }

    fn display(&mut self) -> String {
        let mut s = String::new();
        for r in &self.values {
            for v in r {
                s = format!("{} {}", s, v);
            }
            s = format!("{}\n", s);
        }
        s
    }
}
