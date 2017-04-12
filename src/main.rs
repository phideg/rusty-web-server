extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_minihttp;
extern crate futures;
extern crate num_cpus;
extern crate clap;
extern crate mime_guess;

use std::str;
use std::io;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use futures::future;
use tokio_service::Service;
use tokio_proto::TcpServer;
use tokio_minihttp::{Request, Response, Http};
use clap::App;

struct RustyWebServer;

impl RustyWebServer {
    fn path_for_request(&self, request_path: &str) -> Option<PathBuf> {
        // This is equivalent to checking for hyper::RequestUri::AbsoluteUri
        if !request_path.starts_with("/") {
            return None;
        }
        // Trim off the url parameters starting with '?'
        let end = request_path.find('?').unwrap_or(request_path.len());
        let request_path = &request_path[0..end];
        // Append the requested path to the root directory
        let mut path = PathBuf::from(".");
        if request_path.starts_with('/') {
            path.push(&request_path[1..]);
        } else {
            return None;
        }
        // Maybe turn directory requests into index.html requests
        if request_path.ends_with('/') {
            path.push("index.html");
        }
        Some(path)
    }
}

impl Service for RustyWebServer {
    type Request = Request;
    type Response = Response;
    type Error = std::io::Error;
    type Future = future::Ok<Response, std::io::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut resp = Response::new();

        let path = if let Some(path) = self.path_for_request(&req.path()) {
            path
        } else {
            resp.status_code(404, "Not Found");
            return future::ok(resp);
        };

        let content_type = format!("{}", mime_guess::guess_mime_type(path.clone()));

        match File::open(path) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                match file.read_to_end(&mut buf) {
                    Ok(_) => {
                        resp.header("Content-Type", content_type.as_str())
                            .body(unsafe { str::from_utf8_unchecked(buf.as_slice()) });
                    }
                    Err(e) => {
                        resp.status_code(500, e.to_string().as_str());
                    }
                }
            }
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        resp.status_code(404, "Not Found");
                    }
                    _ => {
                        resp.status_code(500, e.to_string().as_str());
                    }
                }
            }
        }
        future::ok(resp)
    }
}


fn main() {
    let matches = App::new("rusty-web-server")
        .version("0.1")
        .about("A basic HTTP file server")
        .args_from_usage("[ADDR] 'Sets the IP:PORT combination (default \"127.0.0.1:8080\")'")
        .get_matches();

    let addr = matches.value_of("ADDR").unwrap_or("127.0.0.1:8080").parse().unwrap();

    // Display the configuration to be helpful
    println!("addr: http://{}", addr);

    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());
    srv.serve(|| Ok(RustyWebServer))
}
