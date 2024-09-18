#[allow(unused_imports)]
mod response;
mod file;
mod request;

use std::{env, io};
use std::io::{BufRead, Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread::spawn;
use crate::response::{Response, ResponseType};


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
    // process with String or bytes?
    let mut buffer = [0; 2048]; // by bytes
    _stream.read(&mut buffer).unwrap();
    let request_str = std::str::from_utf8(&buffer).unwrap();  // by string
    let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();
    let request_line = lines.first().unwrap().to_string();
    let uri: Vec<&str> = request_line.split(" ").collect();

    let mut res = match uri[1] {
        "/" => {
            Response::new(&"200", &"OK", "", ResponseType::PlainText)
        }
        res if res.starts_with("/echo/") => {
            let parameter: Vec<&str> = res.split("/echo/").collect();
            Response::new(&"200", &"OK", parameter[1], ResponseType::PlainText)
        }
        res if uri[0] == "GET" && res.starts_with("/user-agent") => {
            let mut user_agent = String::new();
            for line in lines {
                if line == "\r\n" || line.is_empty() {
                    break;
                }

                if line.starts_with("User-Agent: ") {
                    user_agent = line.split("User-Agent: ").collect()
                }
            }
            Response::new(&"200", &"OK", user_agent.as_str(), ResponseType::PlainText)
        }
        res if uri[0] == "GET" && res.starts_with("/files/") => {
            let parameters: Vec<&str> = res.split("/files/").collect();
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
        res if uri[0] == "POST" && res.starts_with("/files/") => {
            let parameters: Vec<&str> = res.split("/files/").collect();
            let file_name: &str = parameters[1];
            let env_args: Vec<String> = env::args().collect();
            let mut dir = env_args[2].clone();
            // let mut dir = ".\\".to_string();
            dir.push_str(file_name);

            let mut contents_length: usize = 0;
            for line in lines {
                if line.starts_with("Content-Length: ") {
                    let mut tmp_size = String::new();
                    tmp_size = line.split("Content-Length: ").collect();
                    contents_length = tmp_size.parse().unwrap();
                }
            }
            let mut contents: Vec<u8> = Vec::new();
            if let Some(pos) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
                contents.extend_from_slice(&buffer[pos + 4..pos + 4 + contents_length]); // 헤더를 건너뛰고 내용만 추출
                Ok(contents.clone())
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidData, "No boundary found"))
            }.unwrap();
            match file::write_file(dir, &contents) {
                Ok(file_content) => {
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

    // run command
    _stream.write(res.build().as_slice())
        .expect("panic message!");
}