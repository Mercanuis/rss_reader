use csv::StringRecord;

use crate::readers::Reader;

use super::MAX_LENGTH;

pub struct CsvReader {
    values: Vec<Vec<String>>,
}

impl CsvReader {
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
    fn trim(self) {
        for mut row in self.values {
            for (_i, e) in row.iter_mut().enumerate() {
                if e.len().ge(&MAX_LENGTH) {
                    *e = e[..MAX_LENGTH].to_string();
                }
            }
        }
    }

    fn replace(self, from: &str, to: &str) {
        for mut row in self.values {
            for (_i, e) in row.iter_mut().enumerate() {
                *e = e.replace(from, to);
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
