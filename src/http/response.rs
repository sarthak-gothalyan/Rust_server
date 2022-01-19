// Imports
use super::status_code::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};

// Response struct
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }

    // Send response to socket
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        // Create response body
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        // Write to socket
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, 
            self.status_code.reason_phrase(), 
            body
        )
    }
}