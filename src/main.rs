mod response;

use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use crate::response::Response;


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();


    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let buf_reader = BufReader::new(&mut _stream);
                let request_line = buf_reader.lines().next().unwrap().unwrap();

                let mut res = match request_line.as_str() {
                    "GET / HTTP/1.1" => { Response::new(&"200", &"OK") },
                    _ =>  { Response::new(&"404", &"Not Found") },
                };

                // run command
                _stream.write(res.build().as_slice())
                    .expect("panic message!");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
