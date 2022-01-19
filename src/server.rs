//#![allow(dead_code)]

// Imports
use std::io::Read;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::net::TcpListener;

// Request handler trait
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    } 
}

// Server structure
pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server {
            addr
        }
    }

    pub fn run(&self, mut handler: impl Handler) {
        // Connect to socket
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on port: {}", self.addr);

        loop {
            // Accept Request
            match listener.accept() {
                Ok((mut stream, _)) => {
                    println!("Request accepted");

                    let mut buffer = [0; 1024];
                    
                    // Read  from connection socket
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request read");
                            
                            // Get parsed response
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            // Response failed
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send Response: {}", e);
                            }
                        }
                        // Read failed from connection socket
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                // Request acceptance failed
                Err(e) => {
                    println!("Failed to establish connection: {}", e);
                }
            }
        }
    }
}