use std::env;

use crate::server::ReplicationRole;

#[derive(Clone, Debug)]
pub struct SlaveArgs {
    pub master_host: String,
    pub master_port: i16,
}

impl SlaveArgs {
    pub fn new(master_host: String, master_port: i16) -> Self {
        Self { master_host, master_port }
    }
}


pub struct Args {
    pub port: i16,
    pub role: ReplicationRole,
    pub slave_args: Option<SlaveArgs>,
}

impl Args {
    pub fn from_env() -> Self {
        let args:Vec<String> = env::args().collect();
        let port = Self::get_port(&args);
        let slave_args: Option<SlaveArgs> = Self::get_slave_args(&args);
        let role = match slave_args {
            Some(_) => ReplicationRole::Slave,
            None => ReplicationRole::Master
        };
        Self { port, role, slave_args }
    }

    fn get_port(args: &Vec<String>) -> i16 {
        let default: i16 = 6379;
        match args.iter().position(|arg| arg == "--port") {
            Some(idx) => args.get(idx + 1)
            .and_then(|v| v.parse().ok()).unwrap_or(default),
            None => default
        }
        
    }

    fn get_slave_args(args: &Vec<String>) -> Option<SlaveArgs> {
        args.iter().position(|arg| arg == "--replicaof").and_then(|idx| {
            args.get(idx + 1)
        }).and_then(|host_and_port_str| {
            let host_and_port: Vec<&str> = host_and_port_str.split(" ").collect();
            if host_and_port.len() != 2 {
                return None;
            }
            let host = host_and_port[0].to_string();
            match host_and_port[1].parse() {
                Ok(port) => Some(SlaveArgs::new(host, port)),
                _ => None,
            }
        })
    }
}