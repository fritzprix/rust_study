/**
 * https://www.rfc-editor.org/rfc/rfc8216.html
 */

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
enum HDCP {
    Type0,
    None
}

#[derive(Debug)]
enum VariatnStreamAttribute {
    Bandwidth(u32),
    AvgBandwidth(u32),
    Codec(String),
    Resolution((u32,u32)),
    FrameRate(f32),
    HdcpLevel(HDCP),
    Audio(String),
    Video(String),
    Subtitles(String),
    ClosedCaptions(String)
}

#[derive(Debug)]
enum MasterTag {
    Version(u8),
    Media { mtype: MediaType, uri:Option<String>, name: String, group_id: String, default: bool, channel:u8},
    VariantStream {attributes: Vec<VariatnStreamAttribute>, uri: String}
}

type PlaylistFormatError = Box< dyn std::error::Error>;

impl MasterTag {
    fn parse(tag_str: &str) -> Result<Self, PlaylistFormatError> {
        if tag_str.contains("EXT-X-VERSION") {
            let v: Vec<&str> = tag_str.split(":").collect();
            return Ok(Self::Version(v[1].trim().parse().unwrap()));
        } else if tag_str.contains("EXT-X-MEDIA") {

        } else if tag_str.contains("EXT-X-STREAM-INF") {
            if let Some((_, value)) = tag_str.split_once(":") {
                if let Some((cs_attrs, uri)) = value.split_once("\n") {
                    println!("{}", cs_attrs);
                    let attributes: Vec<VariatnStreamAttribute> = cs_attrs.split(",")
                        .map(|attr| VariatnStreamAttribute::try_from(attr))
                        .filter(|attr| attr.is_ok())
                        .map(|attr| attr.unwrap())
                        .collect();
                    return Ok(Self::VariantStream { attributes, uri: uri.to_string() })
                }
            }
        }
        Err(PlaylistFormatError::from("invalid master playlist tag"))
    }
}

impl TryFrom<&str> for VariatnStreamAttribute {
    type Error = PlaylistFormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().split_once("=") {
            Some(("BANDWIDTH", v)) => {
                Ok(VariatnStreamAttribute::Bandwidth(v.trim().parse().unwrap()))
            },
            Some(("CODECS", codec)) => Ok(VariatnStreamAttribute::Codec(codec.to_string())),
            Some(("RESOLUTION", res)) => {
                let lower_res = res.to_lowercase();
                if lower_res.contains("x") {
                    let (width, height) = lower_res.split_once("x").unwrap();
                    Ok(VariatnStreamAttribute::Resolution((width.parse().unwrap(), height.parse().unwrap())))    
                } else {
                    Err(PlaylistFormatError::from(format!("invalid resolution format {}", lower_res)))
                }
            },
            _ => Err(PlaylistFormatError::from(format!("invalid attribute {} for variant stream", value)))
        }
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
                                          .filter(|tag| tag.is_ok())
                                          .map(|some_tag| some_tag.unwrap())
                                          .into_iter().collect();
        for tag in v {
            println!("=> {:?}", tag);
        }
        pl
    }
}

