#[allow(unused_imports)]
mod response;
mod file;
mod request;

use std::{env};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;
use crate::request::{HttpMethod, Request};
use crate::response::{Response, ResponseType};

const BUFFER_SIZE: usize = 2048;
const ACCEPT_ENCODINGS: [&str; 1] = ["gzip"];


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();


    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                spawn(|| handle_connection(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut _stream: TcpStream) {
    // read buffer
    let mut buffer = [0; BUFFER_SIZE];
    let mut data = Vec::new();
    loop {
        let n = _stream.read(&mut buffer).unwrap();

        if n == 0 {
            break;
        }
        data.extend_from_slice(&buffer[..n]);
        if n < BUFFER_SIZE {
            break;
        }
    }
    let request = Request::from_buffer(data);

    let mut res = match request.uri.as_str() {
        "/" => {
            Response::new(&"200", &"OK", "", ResponseType::PlainText)
        }
        uri if uri.starts_with("/echo/") => {
            let parameter: Vec<&str> = uri.split("/echo/").collect();
            Response::new(&"200", &"OK", parameter[1], ResponseType::PlainText)
        }
        uri if request.method == HttpMethod::Get && uri.starts_with("/user-agent") => {
            let mut user_agent = String::new();
            if request.headers.contains_key("User-Agent") {
                user_agent = request.headers["User-Agent"].clone();
            }
            Response::new(&"200", &"OK", user_agent.as_str(), ResponseType::PlainText)
        }
        uri if request.method == HttpMethod::Get && uri.starts_with("/files/") => {
            let parameters: Vec<&str> = uri.split("/files/").collect();
            let file_name: &str = parameters[1];
            let env_args: Vec<String> = env::args().collect();
            let mut dir = env_args[2].clone();
            dir.push_str(file_name);
            match file::read_file(dir) {
                Ok(file_content) => {
                    Response::new(&"200", &"OK", String::from_utf8(file_content).unwrap().as_str(), ResponseType::OctetStream)
                }
                Err(_) => {
                    Response::new(&"404", &"Not Found", &"", ResponseType::PlainText)
                }
            }
        }
        uri if request.method == HttpMethod::Post && uri.starts_with("/files/") => {
            let parameters: Vec<&str> = uri.split("/files/").collect();
            let file_name: &str = parameters[1];
            let env_args: Vec<String> = env::args().collect();
            let mut dir = env_args[2].clone();
            // let mut dir = ".\\".to_string();
            dir.push_str(file_name);

            let contents: Vec<u8> = request.body.clone();
            match file::write_file(dir, &contents) {
                Ok(_) => {
                    Response::new(&"201", &"Created", &"", ResponseType::OctetStream)
                }
                Err(_) => {
                    Response::new(&"404", &"Not Found", &"", ResponseType::PlainText)
                }
            }
        }
        _ => {
            Response::new(&"404", &"Not Found", "", ResponseType::PlainText)
        }
    };

    // common processing
    if request.headers.contains_key("Accept-Encoding") {
        let accept_encodings = request.headers["Accept-Encoding"].clone();
        for accept_encoding in accept_encodings.split(", ") {
            if ACCEPT_ENCODINGS.contains(&accept_encoding) {
                res.add_header(format!("Content-Encoding: {}", accept_encoding).as_str());
            }
        }
    }

    // run command
    _stream.write(res.build().as_slice())
        .expect("panic message!");
}