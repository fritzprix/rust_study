use regex::{self, Match};

type MalformedUrlError = Box<dyn std::error::Error>;
const URL_CAPTURE_PATTERN: &'static str = "([^:]+):(//[^/]+)([^?]+)?(\\?[^#]+)?(#[\\S]+)?";
const AUTHORITY_PATTERN: &'static str = "//?([^@]+@)?([^:]+)?(:[0-9]+)?";
#[derive(Debug)]
enum URLScheme {
    HTTPS,
    HTTP,
    FILE,
    FTP,
}

#[derive(Debug)]
pub struct Authority {
    user_info: Option<String>,
    host: Option<String>,
    port: Option<u32>,
}

#[derive(Debug)]
pub struct URL {
    scheme: URLScheme,
    authority: Authority, 
    path: String,
    query: String,
    fragment: String,
}

impl URL {
    pub fn parse(url_str: &str) -> Result<Self, MalformedUrlError> {
        let url_pattern = regex::Regex::new(URL_CAPTURE_PATTERN)?;
        match url_pattern.captures(url_str) {
            Some(cap) => {
                let url_chunks: Vec<&str> = cap.iter().skip(1).map(|o_m| {
                    match o_m {
                        Some(m) => m.as_str(),
                        _ => ""
                    }
                }).collect();
                Ok(URL { scheme: URLScheme::try_from(url_chunks[0])?,
                         authority: Authority::try_from(url_chunks[1])?, 
                         path: String::from(url_chunks[2]), 
                         query: String::from(url_chunks[3]),
                         fragment: String::from(url_chunks[4]) })
            },
            None => Err(MalformedUrlError::from(format!("malformed url string {}", url_str)))
        }
    }
}


impl TryFrom<&str> for Authority {
    type Error = MalformedUrlError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        
        let auth_pattern = regex::Regex::new(AUTHORITY_PATTERN)?;
        match auth_pattern.captures(value) {
            Some(cap) => {
                let auth_chunks: Vec<&str> = cap.iter().skip(1).map(|o_m| {
                    match o_m {
                        Some(m) => m.as_str(),
                        _ => ""
                    }
                }).collect();
                
                let user_info = if auth_chunks[0].is_empty() {
                    None
                } else {
                    Some(String::from(auth_chunks[0]))
                };
                    
                let host = if auth_chunks[1].is_empty() {
                    None
                } else {
                    Some(String::from(auth_chunks[1]))
                };

                let port = if auth_chunks[2].is_empty() {
                    None
                } else {
                    Some(auth_chunks[2][1..].parse()?)
                };

                Ok(Authority { user_info, host, port  })
            },
            _ => Err(MalformedUrlError::from(format!("invalid authority value {}", value)))
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
            _ => Err(MalformedUrlError::from(format!("invalid scheme {}", value)))
        }
    }
}


