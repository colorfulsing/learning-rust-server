use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;

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

        request
    }
}
