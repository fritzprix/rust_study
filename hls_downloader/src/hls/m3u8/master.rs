
use std::io::Read;

use hyper::{Client, StatusCode, body::{HttpBody, Buf}};


type PlaylistFormatError = Box< dyn std::error::Error>;

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
enum MediaType {
    Audio,
    Video,
    Subtitles, //< Subtitles
    CCs,       //< Closed-captions
}

#[derive(Debug)]
enum MediaPlaylistAttribute {
    Type(MediaType),
    Uri(String),
    GroupId(String),
    Language(String),
    AssocLanguage(String),
    Name(String),
    Default(bool),
    AutoSelect(bool),
    Forced(bool),
    InstreamId(String)
}

#[derive(Debug)]
enum HDCP {
    Type0,
    None
}


#[derive(Debug)]
pub enum Tag {
    Version(u8),
    MediaPlaylist (Vec<MediaPlaylistAttribute>),
    VariantStream { attributes: Vec<VariatnStreamAttribute>, uri: String }
}

#[derive(Debug)]
pub struct Playlist {
    base_url: String,
    tags: Vec<Tag>
}

impl TryFrom<&str> for HDCP {
    type Error = PlaylistFormatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "TYPE-0" => Ok(Self::Type0),
            "NONE" => Ok(Self::None),
            _ => Err(PlaylistFormatError::from(format!("invalid hdcp level {}", value)))
        }
    }
}

impl TryFrom<&str> for MediaType {
    type Error = PlaylistFormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_uppercase().as_str() {
            "AUDIO" => Ok(Self::Audio),
            "VIDEO" => Ok(Self::Video),
            "SUBTITLES" => Ok(Self::Subtitles),
            "CLOSED-CAPTIONS" => Ok(Self::CCs),
            _ => Err(PlaylistFormatError::from(format!("unknown media type : {}", value)))
        }    
    }
}

impl TryFrom<&str> for VariatnStreamAttribute {
    type Error = PlaylistFormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_uppercase().split_once("=") {
            Some(("BANDWIDTH", v)) => Ok(Self::Bandwidth(v.trim().parse().unwrap())),
            Some(("AVERAGE-BANDWIDTH", v)) => Ok(Self::AvgBandwidth(v.trim().parse().unwrap())),
            Some(("CODECS", codec)) => Ok(Self::Codec(codec.to_string().replace("\"", ""))),
            Some(("FRAME-RATE",rate)) => Ok(Self::FrameRate(rate.parse().unwrap())),
            Some(("HDCP-LEVEL", level)) => Ok(Self::HdcpLevel(HDCP::try_from(level).unwrap())),
            Some(("AUDIO", audio_gid)) => Ok(Self::Audio(audio_gid.to_string().replace("\"", ""))),
            Some(("VIDEO", video_gid)) => Ok(Self::Video(video_gid.to_string().replace("\"", ""))),
            Some(("SUBTITLES", gid)) => Ok(Self::Subtitles(gid.to_string().replace("\"", ""))),
            Some(("CLOSED-CAPTIONS", gid)) => Ok(Self::ClosedCaptions(gid.to_string().replace("\"", ""))),
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

impl TryFrom<&str> for MediaPlaylistAttribute {
    type Error = PlaylistFormatError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_string().split_once("=") {
            Some(("TYPE", mtype)) => Ok(Self::Type(MediaType::try_from(mtype)?)),
            Some(("GROUP-ID", gid)) => Ok(Self::GroupId(gid.to_uppercase().replace("\"", ""))),
            Some(("LANGUAGE", lang)) => Ok(Self::Language(lang.to_uppercase().replace("\"", ""))),
            Some(("ASSOC-LANGUAGE", alang)) => Ok(Self::AssocLanguage(alang.to_uppercase().replace("\"", ""))),
            Some(("NAME", name)) => Ok(Self::Name(name.to_uppercase().replace("\"", ""))),
            Some(("DEFAULT", is_default)) => Ok(Self::Default(is_default == "YES")),
            Some(("URI", uri)) => Ok(Self::Uri(uri.to_string().replace("\"", ""))),
            Some(("AUTOSELECT", is_autosel)) => Ok(Self::AutoSelect(is_autosel == "YES")),
            Some(("FORCED", is_forced)) => Ok(Self::Forced(is_forced == "YES")),
            Some(("INSTREAM-ID", sid)) => Ok(Self::InstreamId(sid.to_uppercase().replace("\"",""))),
            _ => Err(PlaylistFormatError::from(format!("unknown attribute {}", value)))
        }
    }
}


impl Tag {
    fn parse(tag_str: &str) -> Result<Self, PlaylistFormatError> {
        if tag_str.contains("EXT-X-VERSION") {
            let v: Vec<&str> = tag_str.split(":").collect();
            return Ok(Self::Version(v[1].trim().parse().unwrap()));
        } else if tag_str.contains("EXT-X-MEDIA") {
            if let Some((_, attrs)) = tag_str.split_once(":") {
                let attributes: Vec<MediaPlaylistAttribute> = attrs.split(",")
                    .map(|attr| MediaPlaylistAttribute::try_from(attr))
                    .filter(|attr| attr.is_ok())
                    .map(|attr| attr.unwrap())
                    .collect();

                return Ok(Self::MediaPlaylist(attributes));
            }
        } else if tag_str.contains("EXT-X-STREAM-INF") {
            if let Some((_, value)) = tag_str.split_once(":") {
                if let Some((cs_attrs, uri)) = value.split_once("\n") {
                    let attributes: Vec<VariatnStreamAttribute> = cs_attrs.split(",")
                        .map(|attr| VariatnStreamAttribute::try_from(attr))
                        .filter(|attr| attr.is_ok())
                        .map(|attr| attr.unwrap())
                        .collect();

                    return Ok(Self::VariantStream { attributes, uri: uri.trim().to_string() })
                }
            }
        }
        Err(PlaylistFormatError::from("invalid master playlist tag"))
    }
}


impl Playlist {

    pub fn get_variants(&self) -> Vec<&Tag> {
        let mut variants = Vec::new();
        for tag in &self.tags {
            match tag {
                Tag::VariantStream { .. } => {
                    variants.push(tag);
                }
                _ => {
                    continue;
                }
            }
            
        }
        variants
    }
}

impl TryFrom<&str> for Playlist {
    type Error = PlaylistFormatError;
    fn try_from(master_playlist_url: &str) -> Result<Self, Self::Error> {

        let chunked_url: Vec<&str>  = master_playlist_url.split("/").collect();
        let base_url = match chunked_url.split_last() {
            Some((_, url_chunks)) => url_chunks.join("/"),
            _ => String::new()
        };

        let rt = tokio::runtime::Runtime::new()?;

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        
        let client: Client<_, hyper::Body> = Client::builder().build(https);
        let mut res = rt.block_on(client.get(master_playlist_url.parse()?))?;
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
        let tags: Vec<Tag> = playlist.split("#")
                                          .filter(|s| {s.contains("EXT")})
                                          .map(|tag| Tag::parse(tag))
                                          .filter(|tag| tag.is_ok())
                                          .map(|some_tag| some_tag.unwrap())
                                          .into_iter().collect();
        Ok(Playlist { tags, base_url })
    }
}
