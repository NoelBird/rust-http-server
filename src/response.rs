
pub(crate) struct Response {
    status: Vec<u8>,
    header: Vec<u8>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(status_code: &str, message: &str, response_body: &str) -> Self {
        let mut response = Self {
            status: vec![],
            header: vec![],
            body: vec![],
        };
        response.build_status(status_code, message);
        response.build_header(response_body);
        response.build_body(response_body);
        response
    }
    pub fn build(&mut self) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.extend(&self.status);
        ret.extend(&self.header);
        ret.extend(&self.body);
        ret
    }

    pub fn build_status(&mut self, status_code: &str, message: &str) {
        let status_string = format!("HTTP/1.1 {} {}\r\n", status_code, message);
        self.status = Vec::from(status_string.as_bytes());
    }

    pub fn build_header(&mut self, response_body: &str) {
        let content_type = "Content-Type: text/plain\r\n".to_string();
        let content_length = format!("Content-Length: {}\r\n", response_body.len());
        self.header = [content_type, content_length, "\r\n".to_string()].concat().as_bytes().to_vec();
    }

    fn build_body(&mut self, response_body: &str) {
        self.body = format!("{}\r\n", response_body).as_bytes().to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build() {
        let response = Response::build();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\n\r\n")
    }
}