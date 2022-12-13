
use std::io::Read;

use futures;

use hyper::{Client, StatusCode, body::{HttpBody, Buf}};

#[derive(Debug)]
pub struct Playlist {
    version: u8,
}

#[derive(Debug)]
enum MediaType {
    Audio,
    Video, 
    Subtitle,
    ClosedCaption
}

#[derive(Debug)]
enum MasterTag {
    Version(u8),
    Media { mtype: MediaType, uri:Option<String>, name: String, group_id: String, default: bool, channel:u8},
    VariantStream {bandwidth: u32, codecs: String, uri: String}
}

impl MasterTag {
    fn parse(tag_str: &str) -> Option<Self> {
        if tag_str.contains("EXT-X-VERSION") {
            let v: Vec<&str> = tag_str.split(":").collect();
            return Some(Self::Version(v[1].trim().parse().unwrap()));
        } else if tag_str.contains("EXT-X-MEDIA") {

        } else if tag_str.contains("EXT-X-STREAM-INF") {
            match tag_str.split_once(":") {
                Some(s) if matches!(s.1.split_once("\n"), Some(abs)) => {
                },
                _ => {}
            }
        }
        None
    }
}


impl From<String> for Playlist {
    fn from(hostname: String) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut pl = Playlist { version: 0};

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        
        let client: Client<_, hyper::Body> = Client::builder().build(https);
        let mut res = rt.block_on(client.get(hostname.parse().unwrap())).unwrap();
        let playlist = match res.status() {
            StatusCode::OK => {
                let mut body_message = String::new();
                while let Some(data) = futures::executor::block_on(res.body_mut().data()) {
                    let body = match data {
                        Ok(raw) => raw,
                        Err(e) => {
                            println!("error on downloading playlist {}", e);
                            std::process::exit(1);
                        }
                    };
                    let mut reader = body.reader();
                    if let Ok(sz) = reader.read_to_string(&mut body_message) {
                        println!("{} bytes of string is read", sz);
                    }
                }
                body_message
            },
            code => {
                println!("can't get HLS playlist (code: {})", code);
                std::process::exit(1);
            }
        };
        let v: Vec<MasterTag> = playlist.split("#")
                                          .filter(|s| {s.contains("EXT")})
                                          .map(|tag| MasterTag::parse(tag))
                                          .filter(|tag| tag.is_some())
                                          .map(|some_tag| some_tag.unwrap())
                                          .into_iter().collect();
        for tag in v {
            println!("=> {:?}", tag);
        }
        pl
    }
}

