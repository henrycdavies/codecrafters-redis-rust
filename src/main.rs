pub mod args;
pub mod resp;
pub mod pool;
pub mod server;
pub mod store;
pub mod util;

use std::{io::{Read, Result, Write}, net::{TcpListener, TcpStream}, sync::Arc};

use args::Args;
use resp::{command::CommandHandler, Array, Command, RESPDataType, StoredValue, COMMAND_INDICATOR};
use pool::ThreadPool;
use server::Info;
use store::{create_key_value_store, KeyValueStore};

use crate::server::{ReplicationInfo, ReplicationRole};

fn main() {
    let args = Args::from_env();
    let addr = format!("127.0.0.1:{}", args.port);
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);
    let store: KeyValueStore<StoredValue> = create_key_value_store();
    let server_info = Arc::new(Info::new());
    for stream in listener.incoming() {
        if let Ok(_stream) = stream {
            let _server_info = Arc::clone(&server_info);
            let _store = Arc::clone(&store);
            pool.execute(|| {
                handle_stream(_stream, _store, _server_info);
            });
        }
    }
}

fn clear_buffer(buf: &mut [u8]) {
    for i in 0..buf.len() {
        buf[i] = 0;
    }
}

fn handle_stream(mut stream: TcpStream, store: KeyValueStore<StoredValue>, server_info: Arc<Info>) -> Result<()> {
    let mut buf = [0; 512];
    let command_handler = CommandHandler::new(store, server_info);
    while let Ok(_) = stream.read(&mut buf) {
        let response_str = match Array::from_bytes(&buf) {
            Ok(resp_array) => {
                if resp_array.parts[0].indicator == COMMAND_INDICATOR {
                    match Command::from_resp_array(&resp_array.parts) {
                        Ok(command) => {
                            command_handler.execute(command)
                        },
                        _ => {
                            resp::Error::new("ERR unexpected error occurred").into_response_str()
                        },
                    }
                } else {
                    resp::Error::new("ERR unexpected error occurred").into_response_str()
                }
            },
            Err(_) => resp::Error::new("ERR unexpected error occurred").into_response_str()
        };
        stream.write_all(response_str.as_bytes()).unwrap();
        clear_buffer(&mut buf);
    }
    Ok(())
}


