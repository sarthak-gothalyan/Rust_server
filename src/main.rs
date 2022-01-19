#![allow(dead_code)]

// Self modules
mod server;
mod http;
mod website_handler;

// Imports
use server::Server;
use website_handler::WebsiteHandler;
use std::env;
use std::thread;

// Create and start double servers
fn main() {
    let server1 = Server::new("127.0.0.1:8080".to_string());
    let server2 = Server::new("127.0.0.1:8081".to_string());
    let public_path1 = match env::var("PUBLIC_PATH") {
        Ok(val) => val,
        Err(e) => {
            println!("Did not find PUBLIC_PATH: {}", e);
            println!("Using default public path.");
            format!("{}/public", env!("CARGO_MANIFEST_DIR"))
        }
    };
    let public_path2 = match env::var("PUBLIC_PATH") {
        Ok(val) => val,
        Err(e) => {
            println!("Did not find PUBLIC_PATH: {}", e);
            println!("Using default public path.");
            format!("{}/public", env!("CARGO_MANIFEST_DIR"))
        }
    };
    thread::spawn(move || {server1.run(WebsiteHandler::new(public_path1));});
    server2.run(WebsiteHandler::new(public_path2));
}