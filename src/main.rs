mod response;

use std::io::{Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use crate::response::Response;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // let mut buf: [u8; 1000] = [0;1000];
    //listener.accept();


    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                // _stream.read(&mut buf).expect("TODO: panic message");
                let mut res = Response::new();
                _stream.write(res.build().as_slice())
                    .expect("panic message!");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
