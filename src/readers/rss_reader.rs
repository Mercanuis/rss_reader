use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

use reqwest::blocking;
use rss::Channel;

use super::Reader;
use super::MAX_LENGTH;

pub struct RssReader {
    title: String,
    description: String,
}

impl RssReader {
    pub fn new(url: String) -> Self {
        get_rss(url)
            .map(|c| RssReader {
                title: c.title,
                description: c.description,
            })
            .unwrap_or_else(|_| RssReader {
                title: Default::default(),
                description: Default::default(),
            })
    }
}

fn get_rss(url: String) -> Result<Channel, Box<dyn Error>> {
    let content = blocking::get(&url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

impl Display for RssReader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RSS Title - {}, RSS Description - {}",
            self.title, self.description
        )
    }
}

impl Reader for RssReader {
    fn trim(mut self) {
        if self.title.len().ge(&MAX_LENGTH) {
            let s = &self.title.clone()[..MAX_LENGTH];
            self.title = s.to_string();
        }

        if self.description.len().ge(&MAX_LENGTH) {
            let s = &self.description.clone()[..MAX_LENGTH];
            self.description = s.to_string()
        }
    }

    fn replace(mut self, from: &str, to: &str) {
        self.title = self.title.replace(from, to);
        self.description = self.description.replace(from, to);
    }

    fn display(&mut self) -> String {
        let s = format!(
            "RSS Title - {}, RSS Description - {}",
            self.title, self.description
        );
        s
    }
}
