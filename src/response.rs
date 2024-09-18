use std::collections::HashMap;
use crate::file::gzip_encode;

#[derive(Debug)]
pub(crate) struct Response {
    status: String,
    header: HashMap<String, String>,
    body: Vec<u8>,
}

pub enum ResponseType {
    PlainText,
    OctetStream,
}

impl Response {
    pub fn new(status_code: &str, message: &str, response_body: &str, response_type: ResponseType) -> Self {
        Self {
            status: Self::init_status(status_code, message),
            header: Self::init_header(response_type),
            body: Self::init_body(response_body),
        }
    }
    pub fn build(&mut self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.extend(self.build_status());
        self.body = self.build_body();  // body size can be changed.
        let header = self.build_header();
        ret.extend(header);
        if self.body.len() > 0 {
            ret.extend(b"\r\n");
        }
        ret.extend(self.body.clone());
        ret.extend(b"\r\n");
        ret
    }

    pub fn init_status(status_code: &str, message: &str) -> String {
        format!("HTTP/1.1 {} {}\r\n", status_code, message)
    }

    pub fn init_header(response_type: ResponseType) -> HashMap<String, String> {
        let mut header = HashMap::new();
        match response_type {
            ResponseType::PlainText => {
                header.insert("Content-Type".to_string(), "text/plain".to_string());
            }
            ResponseType::OctetStream => {
                header.insert("Content-Type".to_string(),  "application/octet-stream".to_string());
            }
        }
        header
    }

    fn init_body(response_body: &str) -> Vec<u8> {
        format!("{}", response_body).as_bytes().to_vec()
    }

    pub fn add_header(&mut self, key: String, val: String) {
        self.header.insert(key, val);
    }

    fn build_status(&self) -> &[u8] {
        &self.status.as_bytes()
    }

    fn build_header(&mut self) -> Vec<u8> {
        let mut header_str = String::new();
        // content length는 body에 따라서 길이가 변할 수 있어서 마지막에 계산되어야 함
        self.header.insert("Content-Length".to_string(), self.body.len().to_string());
        for header in self.header.clone() {
            let new_header = format!("{}: {}\r\n", header.0, header.1);
            header_str.push_str(new_header.as_str());
        }
        Vec::from(header_str.as_bytes())
    }

    fn build_body(&mut self) -> Vec<u8> {
        match self.header.contains_key("Content-Encoding") {
            true => {
                gzip_encode(self.body.clone()).unwrap()
            }
            false => {
                self.body.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let mut res = Response::new("200", "OK", "", ResponseType::PlainText);
        assert_eq!(res.build(), b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n")
    }
}