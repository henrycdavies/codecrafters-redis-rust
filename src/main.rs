pub mod array;
pub mod command;
pub mod resp;
pub mod pool;
pub mod util;

use std::{io::{Error, Read, Result, Write}, net::{TcpListener, TcpStream}};

use array::RESPArray;
use pool::ThreadPool;
use resp::RESPDataType;

const CRLF: &str = "\r\n";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        if let Ok(_stream) = stream {
            pool.execute(|| {
                handle_stream(_stream);
            });
        }
    }
}

fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let mut buf = [0; 512];
    while let Ok(_) = stream.read(&mut buf) {
        let first_byte = &buf[0..1][0];
        let parts = std::str::from_utf8(&buf).unwrap().split(CRLF).collect::<Vec<&str>>();
        for part in &parts {
            println!("{}", part);
        }
        let response = match first_byte {
            b'*' => RESPArray::from_str_array(&parts).get_response(),
            _ => "UNIMPLEMENTED".to_string()
        };
        if response == "UNIMPLEMENTED" {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "This function is not yet implemented"));
        }
        stream.write_all(response.as_bytes());
    }
    Ok(())
}


