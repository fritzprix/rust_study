use std::{io::Read, fmt::Write};

extern crate url;
mod https;

type Error = Box<dyn std::error::Error>;

trait UrlStream: Read + Write { }



trait UrlOpen {
    fn open(&self) -> Result<Box<dyn UrlStream>, Error>;
}

impl UrlOpen for url::Url {

    fn open(&self) -> Result<Box<dyn UrlStream>, Error>  {
        match self.scheme().to_lowercase().as_str() {
            "https" => Ok(https::open(self)),
            _ => Err(Error::from("value"))
        }
    }
}

