
#[derive(Debug)]
pub(crate) struct Response {
    status: Vec<u8>,
    header: Vec<u8>,
    body: Vec<u8>,
}

pub enum ResponseType {
    PlainText,
    OctetStream,
}

impl Response {
    pub fn new(status_code: &str, message: &str, response_body: &str, response_type: ResponseType) -> Self {
        let mut response = Self {
            status: vec![],
            header: vec![],
            body: vec![],
        };
        response.build_status(status_code, message);
        response.build_header(response_body, response_type);
        response.build_body(response_body);
        response
    }
    pub fn build(&mut self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.extend(&self.status);
        ret.extend(&self.header);
        if self.body.len() > 0 {
            ret.extend(b"\r\n");
        }
        ret.extend(&self.body);
        ret
    }

    pub fn build_status(&mut self, status_code: &str, message: &str) {
        let status_string = format!("HTTP/1.1 {} {}\r\n", status_code, message);
        self.status = Vec::from(status_string.as_bytes());
    }

    pub fn build_header(&mut self, response_body: &str, response_type: ResponseType) {
        let mut header = String::new();
        match response_type {
            ResponseType::PlainText => {
                header.push_str("Content-Type: text/plain\r\n");
            }
            ResponseType::OctetStream => {
                header.push_str("Content-Type: application/octet-stream\r\n");
            }
        }
        let content_length = format!("Content-Length: {}\r\n", response_body.len());
        header.push_str(content_length.as_str());
        self.header = header.as_bytes().to_vec();
    }

    fn build_body(&mut self, response_body: &str) {
        self.body = format!("{}\r\n", response_body).as_bytes().to_vec();
    }

    pub fn add_header(&mut self, header: &str) {
        let custom_header = format!("{}\r\n", header);
        self.header.extend(custom_header.as_bytes());
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