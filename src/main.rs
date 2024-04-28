pub mod array;
pub mod command;
pub mod resp;
pub mod util;

use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

use array::RESPArray;
use resp::RESPDataType;

const CRLF: &str = "\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 512];
    
    let mut conn_open = true;

    while conn_open {
        match stream.read(&mut buf) {
            Ok(_) => {
                let first_byte = &buf[0..1][0];
                let parts = std::str::from_utf8(&buf).unwrap().split(CRLF).collect::<Vec<&str>>();
                for part in &parts {
                    println!("{}", part);
                }
                let response = match first_byte {
                    b'*' => RESPArray::get_response(&parts),
                    _ => "UNIMPLEMENTED"
                };
                if response == "UNIMPLEMENTED" {
                    println!("UNIMPLEMENTED");
                    return;
                }
                stream.write_all(response.as_bytes());

            },
            Err(_) => {
                conn_open = false;
            }
        }
    
    }
}


