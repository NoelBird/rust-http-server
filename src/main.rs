#[allow(unused_imports)]
mod response;
mod file;
mod request;

use std::env;
use std::io::{BufRead, BufReader, Read, Write};
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
    let mut buf_reader = BufReader::new(&mut _stream);
    let request_line = buf_reader.by_ref().lines().next().unwrap().unwrap();
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
            for line in buf_reader.by_ref().lines() {
                let line = line.unwrap();
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
        _ => {
            Response::new(&"404", &"Not Found", "", ResponseType::PlainText)
        }
    };

    // run command
    _stream.write(res.build().as_slice())
        .expect("panic message!");
}