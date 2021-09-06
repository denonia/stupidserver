mod myhttp;

use std::{
    fs, 
    io::{self, BufRead, ErrorKind, Write}, 
    net::TcpStream
};

use bufstream::BufStream;
use http::StatusCode;

use myhttp::{Request, Response, ContentType};

const STATIC_ROOT: &str = "www";

pub fn handle_conn(stream: TcpStream) -> io::Result<()> {
    println!(
        "Connection from {}",
        stream.peer_addr().unwrap()
    );

    let mut buf = BufStream::new(stream);
    let mut request_str = String::new();

    buf.read_line(&mut request_str)?;

    let response = match Request::parse(&mut request_str) {
        Some(request) => {
            println!("{}", request);
            build_response(&request)
        }
        None => Response::bad_request(),
    };

    let bytes = response.to_bytes();
    buf.write_all(&bytes)?;

    Ok(())
}

fn build_response(request: &Request) -> Response {
    let mut response = Response::new();
    if request.method != "GET" {
        response.status = StatusCode::METHOD_NOT_ALLOWED;
    } else {
        resolve_route(&request.path, &mut response);
    }
    response
}

fn resolve_route(path: &String, response: &mut Response) {
    // some serious routing here
    let path = match path.as_str() {
        "/" => "/index.html",
        _ => path
    };
    
    let path = format!("{}{}", STATIC_ROOT, path);

    let contents = fs::read(&path);
    match contents {
        Ok(contents) => {
            response.body = Some(contents);

            let ext = path.split(".").last().unwrap_or_default();
            response.content_type = ContentType::from_ext(ext);
        }
        Err(e) => {
            response.status = match e.kind() {
                ErrorKind::NotFound | 
                ErrorKind::PermissionDenied => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}
