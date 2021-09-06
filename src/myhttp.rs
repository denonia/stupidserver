use std::fmt::Display;

use chrono::prelude::*;

use http::StatusCode;

pub struct Request {
    pub http_version: String,
    pub method: String,
    pub path: String,
    pub time: DateTime<Local>,
}

impl Request {
    pub fn parse(request: &mut String) -> Option<Request> {
        let mut parts = request.split(" ");

        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();
        let http_version = parts.next()?.trim().to_string();

        let time = Local::now();

        Some(Request {
            method,
            path,
            http_version,
            time: time,
        })
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} {} {}", self.time, self.http_version, self.method, self.path)
    }
}

pub struct Response {
    pub body: Option<Vec<u8>>,
    pub status: StatusCode,
    pub content_type: ContentType,
}

impl Response {
    pub fn new() -> Response {
        Response {
            body: None,
            status: StatusCode::OK,
            content_type: ContentType::TEXT,
        }
    }
    pub fn bad_request() -> Response {
        let mut response = Response::new();
        response.status = StatusCode::BAD_REQUEST;
        response
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let result = format!(
            "HTTP/1.1 {} {}\nAllow: GET\nContent-type: {}\n\n",
            self.status.as_str(),
            self.status.canonical_reason().unwrap_or_default(),
            self.content_type.value()
        );

        let mut bytes = result.as_bytes().to_vec();

        if let Some(mut body) = self.body {
            bytes.append(&mut body);
        }

        bytes
    }
}

pub enum ContentType {
    TEXT,
    HTML,
    CSS,
    GIF,
    JPEG,
    PNG,
    SVG,
    ICO,
    XML,
    JSON,
}

impl ContentType {
    pub fn from_ext(ext: &str) -> ContentType {
        match ext {
            "txt" => ContentType::TEXT,
            "htm" => ContentType::HTML,
            "html" => ContentType::HTML,
            "css" => ContentType::CSS,
            "gif" => ContentType::GIF,
            "jpeg" => ContentType::JPEG,
            "jpg" => ContentType::JPEG,
            "png" => ContentType::PNG,
            "svg" => ContentType::SVG,
            "ico" => ContentType::ICO,
            "xml" => ContentType::XML,
            "json" => ContentType::JSON,
            _ => ContentType::TEXT,
        }
    }
    pub fn value(&self) -> &str {
        match *self {
            ContentType::TEXT => "text/plain",
            ContentType::HTML => "text/html",
            ContentType::CSS => "text/css",
            ContentType::GIF => "image/gif",
            ContentType::JPEG => "image/jpeg",
            ContentType::PNG => "image/png",
            ContentType::SVG => "image/svg+xml",
            ContentType::ICO => "image/x-icon",
            ContentType::XML => "application/xml",
            ContentType::JSON => "application/json",
        }
    }
}
