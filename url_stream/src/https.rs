use std::{fmt::Write, io::{Read, BufWriter}};

use hyper::{Client, client::conn};
use url;
use crate::UrlStream;
extern crate hyper_rustls;
extern crate hyper;

struct HttpUrlStream {

}

impl UrlStream for HttpUrlStream {
}


impl Write for HttpUrlStream {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        unimplemented!()
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        unimplemented!()
    }
}

impl Read for HttpUrlStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
}

pub fn open(url: &url::Url) -> Box<dyn UrlStream> {
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .build();

    let client: Client<_, hyper::Body> = Client::builder().build(connector);
    Box::new(HttpUrlStream{})
}