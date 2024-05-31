use std::{io::{Result, Write}, net::TcpStream};
use std::io::Read;

use crate::resp::{array::RESPArrayElement, Array, BulkString, RESPDataType, SimpleString};

use super::BaseCommand;

#[derive(Debug)]
pub struct PingCommand;

impl PingCommand {
    pub fn make(stream: &mut TcpStream) -> Result<String> {
        let ping = BulkString::new("PING").into_response_str();
        match RESPArrayElement::from_str_vec(ping.split("\r\n").collect::<Vec<&str>>()) {
            Ok(elements) => {
                let arr = Array::from_resp_array(elements);
                let to_write = arr.into_bytes();
                if let Err(_) = stream.write_all(&to_write) {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error writing to stream"));
                };

                let mut buf = [0; 512];
                if let Err(_) = stream.read(&mut buf) {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Error reading from stream"));
                };
                match std::str::from_utf8(&buf) {
                    Ok(v) => {
                        println!("Received: {}", v);
                        Ok(v.to_string())
                    },
                    _ => {
                        println!("Error reading from stream");
                        Err(std::io::Error::new(std::io::ErrorKind::Other, "Error reading from stream"))
                    }
                
                }
            },
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "Error creating RESPArrayElement"))
        }
    }
}

impl BaseCommand for PingCommand {
    fn from_resp_array(arr: &Vec<RESPArrayElement>) -> Result<Self> {
        let _ = arr;
        Ok(Self { })
    }
    fn execute(&self) -> Result<String> {
        Ok(SimpleString::new("PONG").into_response_str())
    }
}