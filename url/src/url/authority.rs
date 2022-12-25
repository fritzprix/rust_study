
use std::vec;

use super::ParseError;


const AUTHORITY_PATTERN: &'static str = "//?([^@]+@)?([^:]+)?(:[0-9]+)?";

struct Authority {

}

trait Dog {
    fn bark(&self);
}

impl Authority {
    pub fn parse<V: Dog>(s: &str) -> Result<Self, ParseError> {
        let dogs: Vec<V> = vec![];
        Err(ParseError::InvalidAuthority)
    }
}