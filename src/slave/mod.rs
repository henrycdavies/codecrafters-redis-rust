use std::net::TcpStream;

use crate::{args::SlaveArgs, resp::command::ping::PingCommand};

pub struct Slave {}

impl Slave {
    pub fn new(args: &SlaveArgs) -> Self {
        let addr = format!("{}:{}", args.master_host, args.master_port);
        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                PingCommand::make(&mut stream);
            },
            Err(e) => {
                println!("Error connecting to master: {}", e);
            }
        };
        Self {}
    }
}