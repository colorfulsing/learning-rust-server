use super::{Method, ParseError};
use std::convert::TryFrom;
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

// this one also implements the TryInto into &[u8]
impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    //GET /something?a=1&b=2 HTTP/1.1

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(path[(i+1)..].to_string());
            path = &path[..i];
        }

        Ok(Self{
            path: path.to_string(),
            query_string,
            method
        })
    }
}

fn get_next_word(text: &str) -> Option<(&str, &str)> {
    for (i, c) in text.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&text[..i], &text[i+1..]))
        }
    }

    None
}
