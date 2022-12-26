use std::{
    collections::HashMap,
    str::{Chars, Split},
    vec,
};

use regex;

type MalformedUrlError = Box<dyn std::error::Error>;
const URL_CAPTURE_PATTERN: &'static str = "([^:]+):(//[^/]+)([^?]+)?(\\?[^#]+)?(#[\\S]+)?";
const AUTHORITY_PATTERN: &'static str = "//?([^@]+@)?([^:]+)?(:[0-9]+)?";
const QUERY_CAPTURE_PATTERN: &'static str = "\\??&?([^=]+)=([^&]+)";
const URL_RESERVED: &'static str = "!*'();:@&=+$,/?%#[]";
const URL_ENCODED_PATTERN: &'static str = "%([0-9a-fA-F]{2})";

#[derive(Debug, Copy, PartialEq)]
pub enum URLScheme {
    HTTPS,
    HTTP,
    FILE,
    FTP,
}

impl Clone for URLScheme {
    fn clone(&self) -> Self {
        match self {
            Self::HTTPS => Self::HTTPS,
            Self::HTTP => Self::HTTP,
            Self::FTP => Self::FTP,
            Self::FILE => Self::FILE,
        }
    }
}

pub struct UrlStream {}

impl URLScheme {
    pub fn as_str(&self) -> &str {
        match self {
            Self::FILE => "file",
            Self::FTP => "ftp",
            Self::HTTP => "http",
            Self::HTTPS => "https",
        }
    }
}

#[derive(Debug)]
pub struct Authority {
    user_info: Option<String>,
    host: String,
    port: Option<u32>,
}

impl Authority {
    pub fn to_string(&self) -> String {
        let mut authority = String::new();
        if let Some(user) = &self.user_info {
            authority += &format!("{}@", user);
        }
        authority += &self.host;
        if let Some(p) = self.port {
            authority += &p.to_string();
        }
        authority
    }
}

#[derive(Debug)]
struct Query {
    parameters: HashMap<String, String>,
}

impl Query {
    pub fn to_string(&self) -> String {
        let mut query: Vec<String> = vec![];
        for (key, value) in &self.parameters {
            query.push(format!("{}={}", key, value));
        }
        query.join("&")
    }
}

#[derive(Debug)]
pub struct Url {
    scheme: URLScheme,
    authority: Authority,
    path: Vec<String>,
    query: Query,
    fragment: Option<String>,
}

trait UrlDecoder {
    fn decode_url(&self) -> Result<String, MalformedUrlError>;
}

trait UrlEncoder {
    fn encode_url(&self) -> String;
}

impl UrlEncoder for &str {
    fn encode_url(&self) -> String {
        let enc = self
            .chars()
            .map(|c| {
                if !URL_RESERVED.contains(c) {
                    if c.is_ascii() {
                        String::from(c)
                    } else {
                        // non-ascii character  => utf8
                        // utf8 => percent-encoding
                        let mut dst = vec![0u8; c.len_utf8()];
                        let _ = c.encode_utf8(&mut dst);
                        dst.iter()
                            .map(|c| String::from(format!("%{:x}", c)).to_uppercase())
                            .collect::<Vec<String>>()
                            .join("")
                    }
                } else {
                    String::from(format!("%{:x}", c as u8)).to_uppercase()
                }
            })
            .collect::<Vec<_>>();
        enc.join("")
    }
}

impl UrlDecoder for &str {
    fn decode_url(&self) -> Result<String, MalformedUrlError> {
        let c_seq: Vec<(usize, char)> = self.char_indices().collect();
        let mut decoded = String::new();
        let mut cursor = c_seq.iter();
        while let Some((pos, c)) = cursor.next() {
            match c {
                &'%' => {
                    let (utf8_raw, consumed_size) = precent_hex_to_utf8(&self[*pos..])?;
                    for _ in 1..consumed_size * 3 {
                        let __ = cursor.next();
                    }
                    let utf8_str = String::from_utf8(utf8_raw)?;
                    decoded += &utf8_str;
                }
                _ => decoded.push(*c),
            }
        }
        Ok(decoded)
    }
}

fn precent_hex_to_utf8(s: &str) -> Result<(Vec<u8>, usize), MalformedUrlError> {
    let leading_byte = match (&s[0..1], &s[1..3]) {
        ("%", hex_str) => u8::from_str_radix(hex_str, 16)?,
        _ => return Err(MalformedUrlError::from("not starting with \"%\"")),
    };

    let len = match leading_byte & 0b1111_0000 {
        0b1111_0000 => 4usize,
        0b1110_0000 => 3usize,
        0b1100_0000 => 2usize,
        _ => 1usize,
    };
    let mut decoded = vec![leading_byte];

    for i in 1..len {
        let pos = i * 3;
        let c = match (&s[pos..pos + 1], &s[pos + 1..pos + 3]) {
            ("%", hex_str) => u8::from_str_radix(hex_str, 16)?,
            _ => return Err(MalformedUrlError::from("not starting with \"%\"")),
        };
        decoded.push(c);
    }

    Ok((decoded, len))
}

impl<'a> TryFrom<&str> for Query {
    type Error = MalformedUrlError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        let query_pattern = regex::Regex::new(QUERY_CAPTURE_PATTERN)?;
        for cap in query_pattern.captures_iter(value) {
            match (cap.get(1), cap.get(2)) {
                (Some(k), Some(v)) => {
                    parameters.insert(String::from(k.as_str()), String::from(v.as_str()));
                }
                _ => {
                    return Err(MalformedUrlError::from(format!(
                        "invalid query string {}",
                        value
                    )))
                }
            }
        }
        Ok(Query { parameters })
    }
}

impl TryFrom<&str> for Authority {
    type Error = MalformedUrlError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let auth_pattern = regex::Regex::new(AUTHORITY_PATTERN)?;
        match auth_pattern.captures(value) {
            Some(cap) => {
                let authority_chunks: Vec<&str> = cap
                    .iter()
                    .skip(1)
                    .map(|o_m| match o_m {
                        Some(m) => m.as_str(),
                        _ => "",
                    })
                    .collect();

                let user_info = match authority_chunks[0] {
                    "" => None,
                    user => Some(user.replace("@", "")),
                };

                let host = String::from(authority_chunks[1]);

                let port = match authority_chunks[2] {
                    "" => None,
                    port_str => Some(port_str[1..].trim().parse::<u32>()?),
                };

                Ok(Authority {
                    user_info,
                    host,
                    port,
                })
            }
            _ => Err(MalformedUrlError::from(format!(
                "invalid authority format {}",
                value
            ))),
        }
    }
}

impl TryFrom<&str> for URLScheme {
    type Error = MalformedUrlError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "http" => Ok(Self::HTTP),
            "https" => Ok(Self::HTTPS),
            "file" => Ok(Self::FILE),
            "ftp" => Ok(Self::FTP),
            _ => Err(MalformedUrlError::from(format!("invalid scheme {}", value))),
        }
    }
}

impl Url {
    pub fn parse(url_str: &str) -> Result<Self, MalformedUrlError> {
        let url_pattern = regex::Regex::new(URL_CAPTURE_PATTERN)?;
        match url_pattern.captures(url_str) {
            Some(cap) => {
                let url_chunks: Vec<&str> = cap
                    .iter()
                    .skip(1)
                    .map(|o_m| match o_m {
                        Some(m) => m.as_str(),
                        _ => "",
                    })
                    .collect();

                let fragment = match url_chunks[4] {
                    "" => None,
                    v => Some(v.replace("#", "")),
                };

                Ok(Url {
                    scheme: URLScheme::try_from(url_chunks[0])?,
                    authority: Authority::try_from(url_chunks[1])?,
                    path: url_chunks[2].split("/").map(|s| String::from(s)).collect(),
                    query: Query::try_from(url_chunks[3])?,
                    fragment,
                })
            }
            None => Err(MalformedUrlError::from(format!(
                "malformed url string {}",
                url_str
            ))),
        }
    }

    pub fn join(self, rel_path: &str) -> Self {
        unimplemented!()
    }

    pub fn to_string(&self) -> String {
        unimplemented!()
    }

    pub fn scheme<'a>(&'a self) -> URLScheme {
        self.scheme
    }

    pub fn host<'a>(&'a self) -> &'a str {
        &self.authority.host
    }

    pub fn userinfo<'a>(&'a self) -> Option<&'a str> {
        match &self.authority.user_info {
            Some(v) => Some(&v),
            _ => None,
        }
    }

    pub fn path<'a>(&'a self) -> String {
        self.path.join("/")
    }

    pub fn path_segments<'a>(&'a self) -> Vec<&'a str> {
        self.path.iter().skip(1).map(|seg| seg.as_str()).collect()
    }

    pub fn port(&self) -> Option<u32> {
        self.authority.port
    }

    pub fn query<'a>(&'a self) -> &'a HashMap<String, String> {
        &self.query.parameters
    }

    pub fn fragment<'a>(&'a self) -> Option<&'a str> {
        match &self.fragment {
            Some(v) => Some(v),
            _ => None,
        }
    }

    pub fn open(self) -> Result<UrlStream, std::io::Error> {
        unimplemented!()
    }
}

#[test]
fn test_url_parse() -> Result<(), MalformedUrlError> {
    let url =
        Url::parse("https://user@www.domain.com:3232/path1/path2/path3?name=david&age=23#about")?;
    assert_eq!(url.scheme(), URLScheme::HTTPS);
    assert_eq!(url.userinfo(), Some("user"));
    assert_eq!(url.host(), "www.domain.com");
    assert_eq!(url.port(), Some(3232));
    assert_eq!(url.path(), "/path1/path2/path3");
    assert_eq!(url.path_segments(), vec!["path1", "path2", "path3"]);
    let query_parameters = url.query();
    assert_eq!(query_parameters.get("name"), Some(&"david".to_string()));
    assert_eq!(query_parameters.get("age"), Some(&"23".to_string()));
    assert_eq!(url.fragment(), Some("about"));

    Ok(())
}

#[test]
fn test_pair_endcode_decode_url() {
    assert_eq!(
        encode_url("user@somemail.com:some@@pa??!#$ord"),
        "user%40somemail.com%3Asome%40%40pa%3F%3F%21%23%24ord"
    );
    assert_eq!(
        "user%40somemail.com%3Asome%40%40pa%3F%3F%21%23%24ord"
            .decode_url()
            .unwrap(),
        String::from("user@somemail.com:some@@pa??!#$ord")
    );
    assert_eq!(encode_url("/패스/사용자?이름=아무개&나이=32"), "%2F%ED%8C%A8%EC%8A%A4%2F%EC%82%AC%EC%9A%A9%EC%9E%90%3F%EC%9D%B4%EB%A6%84%3D%EC%95%84%EB%AC%B4%EA%B0%9C%26%EB%82%98%EC%9D%B4%3D32");
    assert_eq!("%2F%ED%8C%A8%EC%8A%A4%2F%EC%82%AC%EC%9A%A9%EC%9E%90%3F%EC%9D%B4%EB%A6%84%3D%EC%95%84%EB%AC%B4%EA%B0%9C%26%EB%82%98%EC%9D%B4%3D32".decode_url().unwrap(), String::from("/패스/사용자?이름=아무개&나이=32"));
}
