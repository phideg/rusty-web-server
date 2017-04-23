extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_minihttp;
extern crate futures;
extern crate num_cpus;
extern crate clap;
extern crate mime_guess;
extern crate url;

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
use url::Url;

struct RustyWebServer;

impl RustyWebServer {
    fn path_for_request(&self, request_path: &str) -> Option<PathBuf> {
        let url = "http://dummy.io".to_string() + request_path;
        let url = Url::parse(url.as_str());
        if url.is_err() {
            return None;
        }
        let decoded_path = url.unwrap();
        let decoded_path = url::percent_encoding::percent_decode(decoded_path.path().as_bytes())
            .decode_utf8_lossy();
        let mut path = PathBuf::from(".".to_string() + decoded_path.as_ref());
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
