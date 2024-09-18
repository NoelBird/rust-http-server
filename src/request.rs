use std::collections::HashMap;
use std::io;

#[allow(unused_imports)]

pub struct Request {
    pub(crate) method: HttpMethod,
    pub(crate) uri: String,
    http_version: String,
    query_params: HashMap<String, String>,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
}

#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    Get,
    Post,
}

impl Request {
    pub fn from_buffer(buffer: Vec<u8>) -> Self {
        let request_str = std::str::from_utf8(&buffer).unwrap();  // by string
        let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();
        let request_line = lines.first().unwrap().to_string();
        let vec_request: Vec<&str> = request_line.split(" ").collect();

        // parse method
        let method_str = vec_request[0];
        let method = match method_str {
            "GET" => {HttpMethod::Get}
            "POST" => {HttpMethod::Post}
            _ => {panic!("Invalid Http Method")}
        };

        // parse uri
        let uri_str = vec_request[1];

        // http version
        let http_version = vec_request[2];

        let headers = Self::parse_headers(lines);

        let body = Self::parse_body(&buffer, &headers);

        Self {
            method,
            uri: uri_str.to_string(),
            http_version: http_version.to_string(),
            query_params: Default::default(),
            headers,
            body,
        }
    }

    fn parse_headers(lines: Vec<String>) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        for line in &lines[1..] {
            if line == "" {
                break
            }
            let splitted: Vec<&str> = line.split(": ").collect();
            let key = splitted[0].to_string();
            let val = splitted[1].to_string();
            hm.insert(key, val);
        }
        hm
    }

    fn parse_body(buffer: &Vec<u8>, headers: &HashMap<String, String>) -> Vec<u8> {
        let mut contents_length: usize = 0;
        if headers.contains_key("Content-Length") {
            contents_length = headers["Content-Length"].clone().parse().unwrap();
        }
        let mut contents: Vec<u8> = Vec::new();
        if let Some(pos) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
            contents.extend_from_slice(&buffer[pos + 4..pos + 4 + contents_length]); // 헤더를 건너뛰고 내용만 추출
            Ok(contents.clone())
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "No boundary found"))
        }.unwrap();
        contents
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use crate::request::{HttpMethod, Request};

    #[test]
    fn test_from_buffer_success() {

    }

    #[test]
    fn test_method_should_get() {
        let buffer = b"GET / HTTP/1.1\r\nHost: localhost:4221\r\n\r\n";
        let request = Request::from_buffer(buffer.into());

        assert_eq!(request.method, HttpMethod::Get);
    }

    #[test]
    fn test_method_should_post() {
        let buffer = b"POST / HTTP/1.1\r\nHost: localhost:4221\r\n\r\n";
        let request = Request::from_buffer(buffer.into());

        assert_eq!(request.method, HttpMethod::Post);
    }

    #[test]
    fn test_uri() {
        let buffer = b"GET / HTTP/1.1\r\nHost: localhost:4221\r\n\r\n";
        let request = Request::from_buffer(buffer.into());

        assert_eq!(request.uri, "/");
    }

    #[test]
    fn test_parse_head_success() {
        let buffer = b"GET / HTTP/1.1\r\nHost: localhost:4221\r\n\r\n";
        let request = Request::from_buffer(buffer.into());

        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert("Host".to_string(), "localhost:4221".to_string());
        assert_eq!(request.headers, hm);
    }

    #[test]
    fn test_parse_body_success() {
        let buffer = b"POST /files/strawberry_pineapple_pineapple_pear HTTP/1.1\r\n\
        Host: localhost:4221\r\nContent-Length: 64\r\nContent-Type: application/octet-stream\r\n\r\n\
        mango pineapple raspberry apple pear raspberry strawberry orange";
        let request = Request::from_buffer(buffer.into());

        let expected = b"mango pineapple raspberry apple pear raspberry strawberry orange";
        assert_eq!(request.body, expected);
    }


}