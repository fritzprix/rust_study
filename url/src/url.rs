use std::{collections::{HashMap}, str::Split};

use regex;

type MalformedUrlError = Box<dyn std::error::Error>;
const URL_CAPTURE_PATTERN: &'static str = "([^:]+):(//[^/]+)([^?]+)?(\\?[^#]+)?(#[\\S]+)?";
const AUTHORITY_PATTERN: &'static str = "//?([^@]+@)?([^:]+)?(:[0-9]+)?";
const QUERY_CAPTURE_PATTERN: &'static str = "\\??&?([^=]+)=([^&]+)";
#[derive(Debug,Copy, PartialEq)]
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

#[derive(Debug)]
pub struct Authority {
    user_info: Option<String>,
    host: String,
    port: Option<u32>,
}

#[derive(Debug)]
struct Query {
    parameters: HashMap<String, String>
}

#[derive(Debug)]
pub struct Url {
    scheme: URLScheme,
    authority: Authority, 
    path: String,
    query: Query,
    fragment: Option<String>,
}


pub struct UrlStream {

}

impl Url {
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

                let fragment = match url_chunks[4] {
                    "" => None,
                    v => Some(v.replace("#", ""))
                };


                Ok(Url { scheme: URLScheme::try_from(url_chunks[0])?,
                         authority: Authority::try_from(url_chunks[1])?, 
                         path: String::from(url_chunks[2]), 
                         query: Query::try_from(url_chunks[3])?,
                         fragment })
            },
            None => Err(MalformedUrlError::from(format!("malformed url string {}", url_str)))
        }
    }
}


impl <'a> TryFrom<&str> for Query {

    type Error = MalformedUrlError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parameters: HashMap<String, String> = HashMap::new();
        let query_pattern = regex::Regex::new(QUERY_CAPTURE_PATTERN)?;
        for cap in query_pattern.captures_iter(value) {
            match (cap.get(1), cap.get(2)) {
                (Some(k), Some(v)) => {
                    parameters.insert(String::from(k.as_str()), String::from(v.as_str()));
                },
                _ => return Err(MalformedUrlError::from(format!("invalid query string {}", value)))
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
                
                let authority_chunks: Vec<&str> = cap.iter().skip(1).map(|o_m| {
                    match o_m {
                        Some(m) => m.as_str(),
                        _ => ""
                    }
                }).collect();


                let user_info = match authority_chunks[0] {
                    "" => None,
                    user => Some(user.replace("@", ""))
                };

                let host = String::from(authority_chunks[1]);

                let port = match authority_chunks[2] {
                    "" => None,
                    port_str => Some(port_str[1..].trim().parse::<u32>()?)
                };

                Ok(Authority { user_info, host, port  })
            },
            _ => Err(MalformedUrlError::from(format!("invalid authority format {}", value)))
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

impl Url {

    pub fn scheme<'a>(&'a self) -> URLScheme {
        self.scheme
    }

    pub fn host<'a>(&'a self) -> &'a str {
        &self.authority.host
    }

    pub fn userinfo<'a>(&'a self) -> Option<&'a str> {
        match &self.authority.user_info {
            Some(v) => Some(&v),
            _ => None
        }
    }

    pub fn path<'a>(&'a self) -> &'a str {
        &self.path
    }

    pub fn path_segments<'a>(&'a self) -> Vec<&'a str> {
        self.path.split("/").skip(1).collect()
    }

    pub fn port(&self) -> Option<u32> {
        self.authority.port
    }

    pub fn query<'a>(&'a self) -> &'a HashMap<String,String> {
        &self.query.parameters
    }

    pub fn fragment<'a>(&'a self) -> Option<&'a str> {
        match &self.fragment {
            Some(v) => Some(v),
            _ => None
        }
    }

    pub fn open(self) -> Result<UrlStream, std::io::Error> {
        unimplemented!()
    }
    
}

#[test]
fn url_parse() -> Result<(),MalformedUrlError> {
    let url = Url::parse("https://user@www.domain.com:3232/path1/path2/path3?name=david&age=23#about")?;
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