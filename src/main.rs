extern crate clap;
extern crate csv;
extern crate reqwest;
extern crate rss;

use crate::readers::determine_reader;
use crate::writers::determine_writer;

mod opts;
mod readers;
mod writers;

fn main() {
    let args = opts::Args::new();

    //Create reader based in INPUT arg
    let mut reader = determine_reader(args.get_url(), args.get_input());

    //Use the TRIM and REPLACE args (if there) to modify readers
    if args.is_trim() {
        reader.trim();
    }
    if let Some(i) = args.get_replace() {
        let r: Vec<&str> = i.split('|').collect();
        //For security, only take the first two args
        reader.replace(r[0], r[1])
    }

    //Create writer based on OUTPUT arg
    if let Some(i) = args.get_output() {
        let mut writer = determine_writer(args.get_url(), i, reader);
        writer.write();
    } else {
        println!("{}", reader.display());
    }
}
