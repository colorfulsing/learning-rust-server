use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::TcpStream;
use std::io::{Write, Result as IOResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self{status_code, body}
    }

    pub fn send(&self, stream: &mut TcpStream) -> IOResult<()> {
        let body = match &self.body {
            Some(value) => value,
            None => ""
        };
        write!(stream, "HTTP/1.0 {} {}\r\n\r\n{}\r\n\r\n", self.status_code, self.status_code.code_reason(), body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let body = match &self.body {
            Some(value) => value,
            None => ""
        };
        write!(f, "HTTP/1.0 {} {}\r\n\r\n{}\r\n\r\n", self.status_code, self.status_code.code_reason(), body)
    }
}